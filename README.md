# Mobile_DNA_Sim

## Project Goals: 
1.Design a scalabale mutable artificial Genome 
- Figure out the best way to efficiently represent genomes from 160K to 120B
- Should be able to handle different types of mobile DNA, copy and paste vs cut and paste
- Would be interesting to tweak paramaters and record the differences in outcomes, such as (probability of mobilizing, number of mobilization events/Time passing/cell replications)

2.Record the interactions of mobile DNA
- Find and record where the mobile DNA inserted (what element it inserted into)
- Will need to be clever about the search approach. 
- I think, it would be most efficient to design a way to scan the array at the end and figure out all of the interactions rather than recording during mobilization, but I could be wrong.
 
3. Once the simulation is working and is fast, it would be interesting to run this simulation across the entire span of genome size ranges using key species as anchor points for gene size, length, and number

### Start

1. I think it would be best to start by using the fly as our template genome

# Fly Genome Statistics and Initial Calculations
genome_size = 180000000
mean_gene_length = 462
min_gene_length = 66
max_gene_length = 14544
num_genes = 14000




