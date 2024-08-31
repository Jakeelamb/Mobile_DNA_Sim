# Mobile_DNA_Sim
![TE_dilution.jpg](https://github.com/Jakeelamb/Mobile_DNA_Sim/blob/main/images/TE_dilution.jpg?raw=true)

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

I will leave the language choice up to you Python vs. Rust

* My one thing to add about this is that Rust is substantially faster and more efficient when it comes to cpu and memory utalization, as well as it natively supports multithreading, so we could utalize ~infitite cpu cores through a HPC. 
* Rust is harder to write as a nooby than python however, so it might take longer to get the ball rolling as far as learning a new language/syntax.

1. I think it would be best to start by using the fly as our template genome

# Fly Genome Statistics and Initial Calculations
genome_size = 180000000
mean_gene_length = 462
min_gene_length = 66
max_gene_length = 14544
num_genes = 14000

# Things to consider

- Probability of moving
	* Mobile DNA do not mobilize every cycle

- Different types of Mobile DNA exist
        * Copy and paste vs cut and paste

- DNA is double stranded, so we need to figure out how to handle each strand being different lengths
	* Ideas: 
		* Set the arrays to be larger and than the actual genome size. Initially set the excess array slots to be blank or a number. Then as array expands/ contracts it can have space to expand but you do not need to reconfigure the arrays memory and this may solve the problem of the strands being different lengths.  

- How to determine where the mobile DNA inserts?
	* Might be best to just assume random coordinate on random strand

- How to calculate interactions:
	* This you might be more knowledgable than I. 
		* Ideas:
			* When the array is initialized, record the coordinates of every element in a file.
			* Then at each mobilization step, record the "random" coordinates that are chosen where it inserts
			* My idea is that if you keep a log of the order of movments then maybe you can calcuate all of the interactions in one single step. 			
			* Alternative ideas are to record snapshots at every "cell cycle", then compare the snapshots. 

