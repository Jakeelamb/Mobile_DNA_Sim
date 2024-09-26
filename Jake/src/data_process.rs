use serde::Deserialize;
use csv::Reader;
use std::error::Error;

#[derive(Clone, Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "Species_name")]
    pub species: String,
    #[serde(rename = "Genome_size")]
    pub genome_size: u64,
    #[serde(rename = "Num_exons")]
    pub exons: u64,
    #[serde(rename = "Assembly")]
    pub assembly: String,
}

pub fn convert_csv_to_list(path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;
    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}