use std::io;
use te_sim::simulation::{run_simulation_with_plot, get_all_species};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the TE Simulation Program!");

    // Get all species
    let all_species = get_all_species()?;

    // Print all species names
    println!("Available species:");
    for (index, species) in all_species.iter().enumerate() {
        println!("{}. {}", index + 1, species.species);
    }

    // Ask user to select a species
    println!("Enter the number of the species you want to simulate (or press Enter for all species):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selected_species = if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= all_species.len() {
            Some(all_species[index - 1].species.as_str())
        } else {
            println!("Invalid selection. Simulating all species.");
            None
        }
    } else {
        None
    };

    // Run the simulation with plot
    run_simulation_with_plot(1000, selected_species)?;

    Ok(())
}
