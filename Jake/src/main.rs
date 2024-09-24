mod data_process;
mod simulation;

use simulation::{run_simulation, SimulationParam};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the simulation for 100 rounds
    let simulation_history = run_simulation(100)?;

    // Print out some results
    for (i, param) in simulation_history.iter().enumerate() {
        println!("Round {}: Species: {}, Active TEs: {}, TEs in Exons: {}, TEs in Non-coding: {}",
                 i + 1,
                 param.get_species(),
                 param.get_active_te(),
                 param.get_te_in_exons(),
                 param.get_te_in_noncoding());
    }

    Ok(())
}