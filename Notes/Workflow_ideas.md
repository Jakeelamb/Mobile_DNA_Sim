# This md file is to outline the workflow and ideas associated with each step of the codebase

# 1. Data collection/generation & Project setup

## 1.1 Create a table with the following columns: 
- [ ] Species
- [ ] Genome_size
- [ ] Number_of_genes
- [ ] Min_gene_length
- [ ] Max_gene_length
- [ ] Number_of_TEs
- [ ] Min_TE_length
- [ ] Max_TE_length

## 1.2 Download the gtf files for the species in the table
- [ ] Pull all of the gtf files from the ensembl website
    ```
    rsync -avhzP --exclude="*abinitio*" rsync://ftp.ensembl.org/ensembl/pub/current_gtf/*/*.gtf.gz .
    ```
- [ ] Unzip the gtf files
    ```
    gunzip *.gz
    ```
- [ ] Parse the gtf files to get the total range of exons
```
cargo run 

## 1.3 Set up the project directory strucutre for a given run
### Function to Set up the project directory strucutre for a given run
// This function should:
// Create a directory called ${species_name}_${date}
// Create a file called Simulation_parameters.txt
// Create a file called Simulation_statistics.txt
// Create a file called Simulation_log.txt
// Date should be formatted as YYYY-MM-DD and include the hh:mm:ss

fn Configure_simulation_dir_structure()

### Function to write the simulation parameters to a file
// This function should export the simulation_parameters struct to a file called Simulation_parameters.txt
```
fn Write_simulation_parameters_to_file()
```
### Function to write console output to a log file
// This function should stream all of the console outputs to a file called Simulation_log.txt
```
fn Write_console_output_to_log_file()
``` 
### Function to write the simulation statistics to a file
// This function should export the simulation_statistics struct to a file called Simulation_statistics.txt
```
fn Write_simulation_statistics_to_file()
```


# 2. Array design and initialization

## 2.1 Structs
### Struct to store the statistics for a species
```
Struct Species_statistics {
    Species: String,
    Genome_size: usize,
    Number_of_genes: u16,
    Min_gene_length: u16,
    Max_gene_length: u32,
    Mean_gene_length: u32,
    Number_of_TEs: u16,
    Min_TE_length: u16,
    Max_TE_length: u32
}
```
### Struct to store the simulation parameters
```
struct Simulation_parameters {
    Species: String,
    Genome_size: usize,
    Current_round: u32,
    Number_of_rounds: u32,
    Number_of_CPU_cores: u8,
    Running_time: u64,
} 
```
### Struct to store the simulation statistics
```
struct Simulation_statistics {
    Number_of_mutations: u32,
    Number_of_TEs_mobilized: u32,
    Number_of_basepairs_added: u64,
    Number_of_TEs_inserted_into_exons: u32,
    Number_of_TEs_inserted_into_introns: u32,
    Number_of_TEs_inserted_into_TEs: u32,
}
```
## 2.2 Functions

# 3. Main simulation/ mobilizing elements


# 4. Recording the interactions


# 5. Data analysis/ visualization
- Generate an html report with the following information: 
    - [ ] - Title: Mobile DNA Simulator
    - [ ] - Statistics
        - [ ] - Species: $Species
        - [ ] - Starting Genome size: $Genome_size
        - [ ] - Ending Genome size: $Genome_size
        - [ ] - Total Simulation rounds ran: $Simulation_rounds
        - [ ] - Number of CPU cores used: $Number_of_CPU_cores
        - [ ] - Total Running time: $Running_time
        - [ ] - Total Memory usage: $Memory_usage
        - [ ] - Number of Mutations: $Number_of_mutations
        - [ ] - Mean number of Mutations per round: $Number_of_mutations / $Simulation_rounds
        - [ ] - Number of TEs mobilized: $Number_of_TEs_mobilized
        - [ ] - Number of basepairs added: $Number_of_basepairs_added
    - [ ] - Graphs

- [ ] Create a tui for the simulation
    - At start, the user should be greeted with a welcome screen and the options to:
        - [ ] Choice 1: Start a new simulation
        - [ ] Choice 2: Exit
    - [ ] If the user chooses to start a new simulation,
        - [ ] Choice 1: Choose a species from the table
        - [ ] Choice 2: Enter custom parameters for the simulation
        - [ ] Choice 3: Return to the previous menu
    - [ ] If the user chooses a species from the table or enters custom parameters:
        - [ ] Choice 1: Number of cell cycles to simulate
        - [ ] Choice 2: Start the simulation
            - [ ] If the number of cell cycles is 0, the program should echo a message and not start the simulation
        - [ ] Choice 2: Return to previous menu
    - [ ] If the user chooses to start the simulation
        - [ ] The user should be taken to the tab 1: Simulation parameters

    - [ ] 3 tabs:
        - [ ] 1. Statistics
            - [ ] - Block widgets:
                - [ ] - Species: $Species
                - [ ] - Genome size: $Genome_size
                - [ ] - Simulation rounds to run: $Simulation_rounds
                - [ ] - Current round: $Current_round
                - [ ] - Running time: $Running_time
                - [ ] - Memory usage: $Memory_usage
                - [ ] - Number of Mutations: $Number_of_mutations
                - [ ] - Number of TEs mobilized: $Number_of_TEs_mobilized
                - [ ] - Number of basepairs added to the genome: $Number_of_basepairs_added
        - [ ] 2. Graphs
            - [ ] - Block 1: Title block: Interactions
            - [ ] - Graph 1: Type=Line Chart,  Title=TEs inserted into Exons, X-axis=Round, Y-axis= Number of interactions
            - [ ] - Graph 2: Type=Line Chart,  Title=TEs inserted into Introns, X-axis=Round, Y-axis= Number of interactions
            - [ ] - Graph 3: Type=Line Chart,  Title=TEs inserted into TEs, X-axis=Round, Y-axis= Number of interactions
        - [ ] 3. Logs
            - [ ] Use the Tui-logger widget