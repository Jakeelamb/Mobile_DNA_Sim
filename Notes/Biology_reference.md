# This file is designed to outline the relevant biological concepts that are important to understand this project

## Terms and definitions
- **Transposable element (TE)**: an element of DNA that can move to a new location within the genome.Also reffered to as "jumping genes" or "mobile DNA"
- **Retrotransposon**: a type of Transposable element that moves by copying and pasting itself in the genome.
- **DNA transposon**: a type of Transposable element that moves by cutting and pasting itself in the genome.
- **Gene**: a region of the DNA that codes for a protein
- **Exon**: basepairs that are vital for protein synthesis, in this simulation we are considering exonic dna to include all the basepairs that pertain to genes. 
- **Intron**: a region of the DNA that is not coded for a protein. Also refered to as "intergenic" DNA or non-coding DNA.
- **Genome**: the entire DNA of an organism, including all the genes and intergenic DNA. Quanitified as total basepairs.
- **Cell cycle**: the process by which a cell replicates its DNA and divides into two daughter cells.
- **Mutation**: a change in the DNA sequence. Mutations can be caused by a variety of mechanisms, including errors in DNA replication, transcription, and translation. Mutations can have a variety of effects on the organism, including changes in protein function, gene expression, and organismal fitness. More often than not, mutations are harmful and detrimental to the organism. 

## Key concepts
- **Transposition**: the process by which a transposable element moves to a new location within the genome. Transposition does not happen every cell cycle, and is dependent on cellular mechanisms that facilitate the movement of the TE. We should consider transposition to be a probabilistic event that occurs and represent it with a probability value in the simulation. 
- **Cell cycle**: Each round of the simulation is symbolic of a cell cycle. In a given simulation, every transposible element will have a probability of being transposed. 
- **Mutation**: Mutations are represented as changes in the DNA sequence. We are interested in recording the total number of mutations that occur over the course of a simulation, and how this changes throughout the simulation. In this experiment if a transposable element inserts into Exonic DNA, we consider this to be a mutation. 
