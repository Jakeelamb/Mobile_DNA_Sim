use rand::Rng;
use std::time::Instant;

#[derive(Debug)]
struct SpeciesData {
    species: String,
    genome_size: u32,
    average_range: u32,
}

#[derive(Debug)]
struct Results {
    species: String,
    beginning_genome_size: u32,
    exon_start_range: u32,
    exon_end_range: u32,
    non_coding_start_range: u32,
    non_coding_end_range: u32,
    active_tes: u32,
    te_mobilized: u32,
    te_static: u32,
    te_in_exons: u32,
    te_in_non_coding: u32,
    exon_new_size: u32,
    non_coding_new_size: u32,
    total_genome_growth: i32,
    simulation_rounds: u32,
    total_time: u128,
}

fn get_random_te_length() -> u32 {
    let mut rng = rand::thread_rng();
    let mean_length = 5000.0; // mean TE length as f64
    let std_dev_length = 2000.0; // standard deviation as f64

    // Generate a normal distribution (clamp the value within a certain range)
    let te_length: f64 = rng.gen::<f64>() * std_dev_length + mean_length;

    // Clamp the result between 100 and 10000, then cast to u32
    te_length.max(100.0).min(10000.0) as u32
}

fn simulation(species_data: Vec<SpeciesData>, simulation_rounds: u32, num_active_te: u32) -> Vec<Results> {
    let mut rng = rand::thread_rng();
    let mut results_list: Vec<Results> = Vec::new();

    for species in species_data.iter() {
        let mut genome_size = species.genome_size;
        let mut exon_range_start = 0;
        let mut exon_range_end = species.average_range;
        let mut nc_range_start = exon_range_end + 1;
        let mut nc_range_end = genome_size;

        let mut te_mobilized = 0;
        let mut te_static = 0;
        let mut te_in_exons = 0;
        let mut te_in_non_coding = 0;

        let start_time = Instant::now();

        for _ in 0..simulation_rounds {
            for _ in 0..num_active_te {
                let te_length = get_random_te_length();
                let te_mobilize_threshold: f64 = 0.5;
                
                if rng.gen::<f64>() < te_mobilize_threshold {
                    let prob_exon = exon_range_end as f64 / genome_size as f64;
                    
                    if rng.gen::<f64>() < prob_exon {
                        exon_range_end += te_length;
                        nc_range_start = exon_range_end + 1;
                        nc_range_end += te_length;
                        te_in_exons += 1;
                    } else {
                        nc_range_end += te_length;
                        te_in_non_coding += 1;
                    }

                    genome_size += te_length;
                    te_mobilized += 1;
                } else {
                    te_static += 1;
                }
            }
        }

        let duration = start_time.elapsed().as_micros(); // elapsed time in microseconds
        
        let result = Results {
            species: species.species.clone(),
            beginning_genome_size: species.genome_size,
            exon_start_range: exon_range_start,
            exon_end_range: exon_range_end,
            non_coding_start_range: nc_range_start,
            non_coding_end_range: nc_range_end,
            active_tes: num_active_te,
            te_mobilized: te_mobilized,
            te_static: te_static,
            te_in_exons: te_in_exons,
            te_in_non_coding: te_in_non_coding,
            exon_new_size: exon_range_end - exon_range_start,
            non_coding_new_size: nc_range_end - nc_range_start,
            total_genome_growth: genome_size as i32 - species.genome_size as i32,
            simulation_rounds: simulation_rounds,
            total_time: duration,
        };

        results_list.push(result);
    }

    results_list
}


fn main () {
    let species_data = vec![
        SpeciesData {
            species: "Species 1".to_string(),
            genome_size: 100000,
            average_range: 1000
        },
        SpeciesData {
            species: "Species 2".to_string(),
            genome_size: 200000,
            average_range: 2000
        },
    ];

    let simulation_rounds = 100;
    let num_active_te = 1000;

    let results = simulation(species_data, simulation_rounds, num_active_te);

    for result in results {
        println!("{:?}", result);
    }
}















// old code 

// mod copy_and_paste;
// mod cut_and_paste;

// use copy_and_paste::CopyAndPaste;
// use cut_and_paste::CutAndPaste;

// fn main() {
//     let cp = CopyAndPaste {
//         sequence: String::from("Copy and Paste"),
//     };

//     let result = cp.print();
//     println!("Result: {}", result);

//     let cut = CutAndPaste {
//         sequence: String::from("Cut and Paste"),
//     };

//     let result = cut.print();
//     println!("Result: {}", result);
// }

