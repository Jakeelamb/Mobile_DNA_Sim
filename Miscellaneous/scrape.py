import requests
from bs4 import BeautifulSoup

species_list_path = "/home/jake/species_list.txt"
base_url = "https://useast.ensembl.org/{}/Info/Annotation"

# Read species list from file
with open(species_list_path, "r") as file:
    species_names = file.read().splitlines()

output_file_path = "/home/jake/species_data.txt"

with open(output_file_path, "w") as output_file:
    for species_name in species_names:
        url = base_url.format(species_name)
        
        # Fetch the web page
        response = requests.get(url)
        
        if response.status_code == 200:
            # Parse the HTML content
            soup = BeautifulSoup(response.text, 'html.parser')
            
            # Find the table with the specified classes
            table = soup.select_one('table.ss.autocenter')
            
            if table:
                # Find all rows with class 'bg1' or 'bg2'
                rows = table.select('tr.bg1, tr.bg2')
                
                output_file.write(f"Data for {species_name}:\n")
                for row in rows:
                    # Extract text from the first two columns
                    columns = row.select('td')
                    if len(columns) >= 2:
                        key = columns[0].text.strip()
                        value = columns[1].text.strip()
                        output_file.write(f"{key}: {value}\n")
                output_file.write("\n")  # Empty line for readability
            else:
                output_file.write(f"Table not found for {species_name}\n\n")
        else:
            output_file.write(f"Failed to fetch data for {species_name}\n\n")

print(f"Data has been written to {output_file_path}")