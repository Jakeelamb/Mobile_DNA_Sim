use rand_distr::{Distribution, Normal};
use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;

// Struct to hold species data
#[derive(Debug)]
struct SpeciesData {
    species: String,
    genome_size: usize,
    average_range: usize,
}

// Function to get the results dictionary as a HashMap
fn get_results_dictionary(
    simulation_rounds: usize,
    genome_size: usize,
    num_active_te: usize,
    range_1_start: usize,
    range_1_end: usize,
    range_2_start: usize,
    range_2_end: usize,
) -> HashMap<String, usize> {
    let mut species_results = HashMap::new();
    // let mut species_results = HashMap::new();
    // species_results.insert("Species".to_string(), species_name);
    species_results.insert("Beginning Genome Size".to_string(), genome_size);
    species_results.insert("Exon Start Range".to_string(), range_1_start);
    species_results.insert("Exon End Range".to_string(), range_1_end);
    species_results.insert("Non-Coding Start Range".to_string(), range_2_start);
    species_results.insert("Non-Coding End Range".to_string(), range_2_end);
    species_results.insert("Active TEs".to_string(), num_active_te);
    species_results.insert("TEs mobilized".to_string(), 0);
    species_results.insert("TEs static".to_string(), 0);
    species_results.insert("TEs in Exons".to_string(), 0);
    species_results.insert("TEs in Non-Coding".to_string(), 0);
    species_results.insert("Exon New Size".to_string(), range_1_end - range_1_start + 1);
    species_results.insert("Non-Coding New Size".to_string(), range_2_end - range_2_start + 1);
    species_results.insert("Total Genome Growth".to_string(), 0);
    species_results.insert("Simulation Rounds".to_string(), simulation_rounds);
    species_results.insert("Total Time".to_string(), 0);
    
    species_results
}

// Function to generate TE lengths
fn get_te_lengths(num_active_te: usize, mean_length: f64, std_dev_length: f64) -> Vec<usize> {
    let normal = Normal::new(mean_length, std_dev_length).unwrap(); // Updated with rand_distr
    let mut rng = rand::thread_rng();
    (0..num_active_te)
        .map(|_| {
            let te_length = normal.sample(&mut rng).max(100.0).min(10000.0);
            te_length as usize
        })
        .collect()
}

// Main simulation function
fn simulation(
    data: Vec<SpeciesData>,
    simulation_rounds: usize,
    num_active_te: usize,
    te_mobilize_threshold: f64,
    num_species: usize,
) -> Vec<HashMap<String, String>> {
    let te_lengths = get_te_lengths(num_active_te, 5000.0, 2000.0);
    let mut results_list = Vec::new();

    for i in 0..num_species {
        let species_data = &data[i];
        let mut genome_size = species_data.genome_size;
        let initial_genome_size = genome_size;
        let average_range = species_data.average_range;

        let exon_range_start = 0;
        let mut exon_range_end = average_range;
        let mut nc_range_start = average_range + 1;
        let mut nc_range_end = genome_size;

        let mut results = get_results_dictionary(
            // species_data.species.clone(),
            simulation_rounds,
            genome_size,
            num_active_te,
            exon_range_start,
            exon_range_end,
            nc_range_start,
            nc_range_end,
        );

        let start_time = Instant::now();

        let mut rng = rand::thread_rng();

        for _ in 0..simulation_rounds {
            for _ in 0..num_active_te {
                let te_length = te_lengths[rng.gen_range(0..te_lengths.len())];
                if rng.gen::<f64>() < te_mobilize_threshold {
                    let te_position = rng.gen_range(0..=genome_size);

                    if te_position <= exon_range_end {
                        exon_range_end += te_length;
                        nc_range_start = exon_range_end + 1;
                        nc_range_end += te_length;
                        *results.get_mut("TEs in Exons").unwrap() += 1;
                    } else {
                        nc_range_end += te_length;
                        *results.get_mut("TEs in Non-Coding").unwrap() += 1;
                    }

                    genome_size += te_length;
                    *results.get_mut("TEs mobilized").unwrap() += 1;
                } else {
                    *results.get_mut("TEs static").unwrap() += 1;
                }
            }
        }

        let end_time = start_time.elapsed().as_secs();
        results.insert("Exon New Size".to_string(), exon_range_end - exon_range_start + 1);
        results.insert(
            "Non-Coding New Size".to_string(),
            nc_range_end - nc_range_start + 1,
        );
        results.insert(
            "Total Genome Growth".to_string(),
            results["Exon New Size"] + results["Non-Coding New Size"] - initial_genome_size,
        );
        results.insert("Total Time".to_string(), end_time as usize);

        let mut result_with_species: HashMap<String, String> = results
            .into_iter()
            .map(|(key, value)| (key, value.to_string()))
            .collect();

        result_with_species.insert("Species".to_string(), species_data.species.clone());

        results_list.push(result_with_species);
    }

    results_list
}

fn main() {
    // Example usage
    let species_data = vec![
        SpeciesData {
            species: "Accipiter_nisus.Accipiter_nisus_ver1.0.112".to_string(),
            genome_size: 1190649881,
            average_range: 533365,
        },
        // SpeciesData {
        //     species: "Species 2".to_string(),
        //     genome_size: 15000,
        //     average_range: 7500,
        // },
    ];

    let results = simulation(species_data, 100, 1000, 0.5, 1);

    for result in results {
        println!("{:?}", result);
    }
}