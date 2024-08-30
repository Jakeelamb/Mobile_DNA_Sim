#Packages:
import numpy as np
import matplotlib.pyplot as plt
from scipy.stats import truncnorm
import logging
import os

#set up logging
logging.basicConfig(filename='simulation.log.txt', filemode='w', level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

#set up directories
if not os.path.exists('figures'):
    os.makedirs('figures')

# Function Definitions
def initialize_grid(grid_height, grid_width):
    grid = np.zeros((grid_height, grid_width), dtype=int)
    return grid

def create_truncated_normal_distribution(mean, min_val, max_val, stdv, num_samples):
    lower, upper = (min_val - mean) / stdv, (max_val - mean) / stdv
    distribution = truncnorm(lower, upper, loc=mean, scale=stdv)
    samples = distribution.rvs(num_samples)
    return samples

# Fly Genome Statistics and Initial Calculations

genome_size = 180000000
mean_gene_length = 462
min_gene_length = 2
max_gene_length = 14544
num_genes = 14000

estimated_std = (max_gene_length - min_gene_length) / 6
exon_lengths = create_truncated_normal_distribution(mean_gene_length, min_gene_length, max_gene_length, estimated_std, num_genes)

retrotransposons_stats = {'mean': 2869, 'min_len': 215, 'max_len': 7490, 'std_dev': 3213}
dnatransposons_stats = {'mean': 2180, 'min_len': 52, 'max_len': 5453, 'std_dev': 2013}

retrotransposons_proportion = 0.18
dnatransposons_proportion = 0.02

genome_minus_exons = genome_size - sum(exon_lengths)
total_retrotransposons_length = genome_minus_exons * retrotransposons_proportion
total_dnatransposons_length = genome_minus_exons * dnatransposons_proportion

num_retrotransposons = int(total_retrotransposons_length / retrotransposons_stats['mean'])
num_dnatransposons = int(total_dnatransposons_length / dnatransposons_stats['mean'])

retrotransposons_lengths = create_truncated_normal_distribution(
    retrotransposons_stats['mean'], retrotransposons_stats['min_len'], 
    retrotransposons_stats['max_len'], retrotransposons_stats['std_dev'], num_retrotransposons)

dnatransposons_lengths = create_truncated_normal_distribution(
    dnatransposons_stats['mean'], dnatransposons_stats['min_len'], 
    dnatransposons_stats['max_len'], dnatransposons_stats['std_dev'], num_dnatransposons)

# Non-Coding Region Calculations

total_coding_length = sum(exon_lengths) + sum(retrotransposons_lengths) + sum(dnatransposons_lengths)
non_coding_length = genome_size - total_coding_length

num_non_coding_segments = 50000
non_coding_segment_lengths = np.random.uniform(low=100, high=non_coding_length/num_non_coding_segments, size=num_non_coding_segments)

while sum(non_coding_segment_lengths) > non_coding_length:
    non_coding_segment_lengths = np.random.uniform(low=100, high=non_coding_length/num_non_coding_segments, size=num_non_coding_segments)

## Plotting Distributions
fig, axs = plt.subplots(4, 1, figsize=(12, 16))

axs[0].hist(exon_lengths, bins=50, color='blue', alpha=0.7)
axs[0].set_title('Exon Lengths')
axs[0].set_xlabel('Length')
axs[0].set_ylabel('Frequency')

axs[1].hist(retrotransposons_lengths, bins=50, color='green', alpha=0.7)
axs[1].set_title('Retrotransposons Lengths')
axs[1].set_xlabel('Length')
axs[1].set_ylabel('Frequency')

axs[2].hist(dnatransposons_lengths, bins=50, color='red', alpha=0.7)
axs[2].set_title('DNA Transposons Lengths')
axs[2].set_xlabel('Length')
axs[2].set_ylabel('Frequency')

axs[3].hist(non_coding_segment_lengths, bins=50, color='purple', alpha=0.7)
axs[3].set_title('Non-Coding Lengths')
axs[3].set_xlabel('Length')
axs[3].set_ylabel('Frequency')

# Save the current figure to the PDF file
plt.savefig('figures/length_distribution_histograms.png')
plt.close()

# Pie Chart of Genome Composition
labels = ['Exons', 'Retrotransposons', 'DNA Transposons', 'Non-Coding']
sizes = [sum(exon_lengths), sum(retrotransposons_lengths), sum(dnatransposons_lengths), non_coding_length]

plt.figure(figsize=(8, 8))
plt.pie(sizes, labels=labels, autopct='%1.1f%%', shadow=True, startangle=140)
plt.title('Genome Composition')

# Save the pie chart figure to the same PDF file
plt.savefig('figures/genome_composition_pie_chart.png')
plt.close()

np.random.seed(42)  # Set a random seed for reproducibility

# Initialize the double-stranded DNA grid
grid = initialize_grid(2, genome_size)  # 2x180000000 grid

# Function to populate the grid with a specific element
def populate_grid(grid, element_lengths, element_type):
    num_placed_elements = 0
    num_dnatransposons_placed =0
    num_retrotransposons_placed = 0
    num_exons_placed = 0
    num_non_coding_segments_placed = 0

    for length in element_lengths:
        length = int(length)
        if length >= grid.shape[1]:
            logging.info(f"Skipping element of length {length} as it exceeds grid size {grid.shape[1]}")
            continue  # Skip this element as it's too long for the grid

        placed = False
        while not placed:
            if grid.shape[1] - length <= 0:
                logging.info(f"Error: length {length} is too large for grid size {grid.shape[1]}")
                break  # Break out of the while loop if this scenario occurs

            strand = np.random.randint(0, 2)  # Choose top (0) or bottom (1) strand randomly
            start_pos = np.random.randint(0, grid.shape[1] - length)
            if grid[strand, start_pos:start_pos + length].sum() == 0:  # Check if the space is empty
                grid[strand, start_pos:start_pos + length] = element_type
                placed = True
                num_placed_elements += 1
                if element_type == 1:
                    num_exons_placed += 1
                elif element_type == 2:
                    num_retrotransposons_placed += 1
                elif element_type == 3:
                    num_dnatransposons_placed += 1
                elif element_type == 4:
                    num_non_coding_segments_placed += 1                #logging.info(f"Placed element of type {element_type} at ({strand}, {start_pos}) to ({strand}, {start_pos + length})")
                #logging.info(f"Total {num_placed_elements} elements of type {element_type} placed on the grid")
                #Log the total number of elements of each type placed on the grid
    logging.info(f"{num_exons_placed} exons placed on the grid")
    logging.info(f"{num_retrotransposons_placed} retrotransposons placed on the grid")
    logging.info(f"{num_dnatransposons_placed} dnatransposons placed on the grid")
    logging.info(f"{num_non_coding_segments_placed} non-coding segments placed on the grid")
    
# Populate the grid with each element type
populate_grid(grid, exon_lengths, 1)  # Exons as type 1
populate_grid(grid, retrotransposons_lengths, 2)  # Retrotransposons as type 2
populate_grid(grid, dnatransposons_lengths, 3)  # DNA transposons as type 3
populate_grid(grid, non_coding_segment_lengths, 0)  

def check_grid_density(grid):
    non_zero_elements = np.count_nonzero(grid)
    total_elements = grid.size
    logging.info(f"Grid Density: {non_zero_elements}/{total_elements} ({non_zero_elements / total_elements * 100}%)")

# After populating the grid
check_grid_density(grid)
