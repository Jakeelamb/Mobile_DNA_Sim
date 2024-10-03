use std::error::Error;
use te_sim::run_simulation;

fn main() -> Result<(), Box<dyn Error>> {
    let num_rounds = 1000; // or however many rounds you want to run
    let final_param = run_simulation(num_rounds)?;
    Ok(())
}