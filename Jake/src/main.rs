use std::error::Error;
use std::io::{self, Write};
use te_sim::{run_simulation, get_all_species};
mod process;

fn main() -> Result<(), Box<dyn Error>> {
    let all_species = get_all_species()?;

    println!("Available species:");
    for (i, species) in all_species.iter().enumerate() {
        println!("{}. {}", i + 1, species.species);
    }
    println!("Enter a number to select a species, or press Enter to run for all species:");

    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim();

    if input.is_empty() {
        println!("Running simulation for all species...");
        run_simulation(100, None)?;
    } else {
        match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= all_species.len() => {
                let selected_species = &all_species[num - 1].species;
                println!("Running simulation for {}...", selected_species);
                run_simulation(1000, Some(selected_species))?;
            }
            _ => {
                println!("Invalid input. Running simulation for all species...");
                run_simulation(100, None)?;
            }
        }
    }

    //process::process_and_archive_results()?;
    
    Ok(())
}