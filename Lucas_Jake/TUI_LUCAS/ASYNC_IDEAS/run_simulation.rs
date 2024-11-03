use rand::distributions::{Distribution, Uniform};
// use rand::Rng;
// use std::sync::atomic::{AtomicUsize, Ordering};

// use std::thread;
// use std::time::Duration;

pub struct SimulationParam {
    pub simulation_round: usize,
    pub genome_size: usize,
    pub exon_end_range: usize,
    // Add other fields as needed
}

impl SimulationParam {
    // Define `new` function
    pub fn new(genome_size: usize, exon_end_range: usize) -> Self {
        SimulationParam {
            simulation_round: 0,
            genome_size,
            exon_end_range,
            // Initialize other fields if necessary
        }
    }

    pub fn run_simulation_round(&mut self) {
        self.simulation_round += 1;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}


// #[derive(Clone, Debug)]
// pub struct SimulationParam {
//     pub genome_size: usize,
//     pub exon_end_range: usize,
//     pub active_te: usize,
//     pub te_mobilized: usize,
//     pub te_in_exons: usize,
//     pub te_in_noncoding: usize,
//     pub simulation_round: usize,
//     pub te_mobilize_prob: f64,
// }

// impl SimulationParam {
//     pub fn new(genome_size: usize, exon_end_range: usize) -> Self {
//         SimulationParam {
//             genome_size,
//             exon_end_range,
//             active_te: 10000,
//             te_mobilized: 0,
//             te_in_exons: 0,
//             te_in_noncoding: 0,
//             simulation_round: 0,
//             te_mobilize_prob: 0.5,
//         }
//     }

//     pub fn run_simulation_round(&mut self) -> usize {
//         self.simulation_round += 1;
//         let mobilized: usize = (self.active_te as f64 * self.te_mobilize_prob).trunc() as usize;
//         self.te_mobilized += mobilized;

//         let te_lengths = self.get_te_lengths(mobilized);
//         let local_te_in_exons = AtomicUsize::new(0);
//         let local_te_in_noncoding = AtomicUsize::new(0);

//         te_lengths.iter().for_each(|_| {
//             let insertion_position: usize = rand::thread_rng().gen_range(0..self.genome_size);
//             if insertion_position > self.exon_end_range {
//                 local_te_in_noncoding.fetch_add(1, Ordering::Relaxed);
//             } else {
//                 local_te_in_exons.fetch_add(1, Ordering::Relaxed);
//             }
//         });

//         self.te_in_exons += local_te_in_exons.load(Ordering::Relaxed);
//         self.te_in_noncoding += local_te_in_noncoding.load(Ordering::Relaxed);
        
//         local_te_in_exons.into_inner() // Return the number of exon insertions as mutations for this round
//     }

//     fn get_te_lengths(&self, num_active_te: usize) -> Vec<usize> {
//         let between = Uniform::from(100..=10000);
//         (0..num_active_te).map(|_| between.sample(&mut rand::thread_rng())).collect()
//     }
// }