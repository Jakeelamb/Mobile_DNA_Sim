import csv
import asyncio
import aiohttp
from bs4 import BeautifulSoup
import os
import time
from aiolimiter import AsyncLimiter
from cachetools import TTLCache

# Use a relative path for the input file
input_file_path = "Database_filename.csv"
output_file_path = "Database_filename_genomesize.csv"

url_templates = {
    "ensembl": "https://useast.ensembl.org/{}/Info/Annotation",
    "plants": "https://plants.ensembl.org/{}/Info/Annotation",
    "fungi": "https://fungi.ensembl.org/{}/Info/Annotation",
    "bacteria": "https://bacteria.ensembl.org/{}/Info/Annotation",
    "protists": "https://protists.ensembl.org/{}/Info/Annotation"
}

# Create a cache with a 1-hour TTL and a maximum of 10000 items
cache = TTLCache(maxsize=10000, ttl=3600)

# Create a rate limiter with 10 requests per second
rate_limiter = AsyncLimiter(10, 1)

async def get_genome_size(session, domain, species_name):
    cache_key = f"{domain}:{species_name}"
    if cache_key in cache:
        return cache[cache_key]

    if domain.lower() not in url_templates:
        print(f"Error: Unknown domain '{domain}' for species '{species_name}'")
        return "N/A"
    
    url = url_templates[domain.lower()].format(species_name)
    
    try:
        async with rate_limiter:
            async with session.get(url) as response:
                response.raise_for_status()
                content = await response.text()
        
        soup = BeautifulSoup(content, 'lxml')
        table = soup.select_one('table.ss.autocenter')
        
        if table:
            rows = table.select('tr.bg1, tr.bg2, tr.bg3')
            
            for row in rows:
                columns = row.select('td')
                if len(columns) >= 2:
                    key = columns[0].text.strip()
                    value = columns[1].text.strip()
                    if key == "Golden Path Length":
                        cache[cache_key] = value
                        return value
        
    except (aiohttp.ClientError, asyncio.TimeoutError) as e:
        print(f"Error fetching data for {species_name}: {e}")
    
    cache[cache_key] = "N/A"
    return "N/A"

async def process_csv():
    if not os.path.exists(input_file_path):
        print(f"Input file not found: {input_file_path}")
        return

    if os.path.exists(output_file_path):
        print(f"Output file already exists: {output_file_path}")
        user_input = input("Do you want to overwrite it? (y/n): ").lower()
        if user_input != 'y':
            print("Operation cancelled.")
            return

    async with aiohttp.ClientSession() as session:
        with open(input_file_path, "r") as input_file, open(output_file_path, "w", newline='') as output_file:
            csv_reader = csv.reader(input_file)
            csv_writer = csv.writer(output_file)

            # Write header
            csv_writer.writerow(["Domain", "Assembly File Name", "Genome Size"])

            # Skip the header row
            next(csv_reader, None)

            total_rows = sum(1 for row in csv.reader(open(input_file_path))) - 1  # Subtract 1 for header
            tasks = []

            for i, row in enumerate(csv_reader, 1):
                if len(row) >= 2:
                    domain, assembly_file_name = row[0], row[1]
                    species_name = assembly_file_name.split('.')[0]  # Assuming species name is before the first dot
                    task = asyncio.create_task(get_genome_size(session, domain, species_name))
                    tasks.append((domain, assembly_file_name, task))

            for i, (domain, assembly_file_name, task) in enumerate(tasks, 1):
                genome_size = await task
                csv_writer.writerow([domain, assembly_file_name, genome_size])
                print(f"Processed {i}/{total_rows} rows", end='\r')

    print(f"\nData has been written to {output_file_path}")

if __name__ == "__main__":
    start_time = time.time()
    asyncio.run(process_csv())
    end_time = time.time()
    print(f"Total execution time: {end_time - start_time:.2f} seconds")