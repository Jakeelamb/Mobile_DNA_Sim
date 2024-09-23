mod data_process;
use data_process::{Record, convert_csv_to_list};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("Data/Species_data.csv");
    let records = convert_csv_to_list(path.to_str().unwrap())?;

    for record in records {
        println!("Species: {}, Genome size: {}, Exons: {}, Assembly: {}",
                 record.species, record.genome_size, record.exons, record.assembly);
    }
    Ok(())
}





