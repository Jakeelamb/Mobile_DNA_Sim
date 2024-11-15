# Mobile_DNA_Sim
![TE_dilution.jpg](https://github.com/Jakeelamb/Mobile_DNA_Sim/blob/main/images/TE_dilution.jpg?raw=true)

## Inspiration
### Conways Game of Life
### [bottom](https://github.com/ClementTsang/bottom)
### [poketex](https://github.com/ckaznable/poketex)

## Project Goals: 
1. Simulate the effect Transposable Elements have variable genome sizes across species 

2. Build a TUI for the simulation
[TUI](https://ratatui.rs/)


## Simualtion Design

### Variables

- Species
  - Name
  - Clade
  - Genome size
  - Num Exons
   
- Transposable elements
  - Number of Active
  - Mean length of TEs
    - Every TEs individual length generated from mean distribution
  - Probability of Death

- Deletion
  - Per basepair deletion probability
  - Sliding window size
    - Sliding window probability of deletion

- Simulation
  - Number of rounds
  - Number of cores

### Function Flow

1. Choose Species or select all species
2. Assign TEs lengths based on distribution and store them in a vector
3. Create Bins for exons and noncoding
4. For round in number of simulation rounds: 
   1. Simulate a round
      1. Mobilize TEs based on parameters
         1. Probability of moving
            1. if Move probability of death
         2. Add length of TEs to respective bin (non coding/ Exons)
      2. Delete DNA
         1. For noncoding BPs
            1. Simulation probability of deletion (small indels)
         2. Divide noncoding BPs into windows of set length (100, 1000, 10000 etc)
            1. Simulate probability of deletion of windows (large indels)


### Outputs

- For every species:
  - Number of TEs inserted in Exons vs Noncoding
  - Number of BP added in Exons vs Noncoding
  - Number of BP deleted by small deletion events (per bp deletion probability) and BP deleted by large deletion events (window deletions)
  - Overall genome size change

### Research explorations: 

- Show 





## Notes
- With the deletions the idea is to try to constrain the unrealistic expansion of genome size
- Goal is to tweak parameters to find plausable values that explain genome size evolution

