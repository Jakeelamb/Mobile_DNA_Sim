use rand::Rng;
use rayon::prelude::*;
use rand_distr::{Normal, Distribution};

#[derive(Clone, Debug)]
pub struct SimulationParam {
    pub genome_size: usize,
    pub exon_length: usize,
    pub noncoding_length: usize,
    pub active_te: usize,
    pub te_mobilized: usize,
    pub te_in_exons: usize,
    pub te_in_noncoding: usize,
    pub simulation_round: usize,
    pub te_mobilize_prob: f64,
    pub exon_bp_deletion_prob: f64,
    pub noncoding_bp_deletion_prob: f64,
    pub exon_window_deletion_prob: f64,
    pub noncoding_window_deletion_prob: f64,
    pub window_size: usize,
    pub bp_deleted_exon: usize,
    pub bp_deleted_noncoding: usize,
}

impl SimulationParam {
    pub fn new(genome_size: usize, exon_size: usize) -> Self {
        SimulationParam {
            genome_size,
            exon_length: exon_size,
            noncoding_length: genome_size.saturating_sub(exon_size),
            active_te: 500, // Default for TUI, can be modified through UI
            te_mobilized: 0,
            te_in_exons: 0,
            te_in_noncoding: 0,
            simulation_round: 0,
            te_mobilize_prob: 0.5,
            exon_bp_deletion_prob: 0.00001,
            noncoding_bp_deletion_prob: 0.001,
            exon_window_deletion_prob: 0.00001,
            noncoding_window_deletion_prob: 0.01,
            window_size: 1000,
            bp_deleted_exon: 0,
            bp_deleted_noncoding: 0,
        }
    }

    pub fn process_deletions(&mut self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        
        // Calculate single base deletions
        let expected_exon_deletions = (self.exon_length as f64 * self.exon_bp_deletion_prob) as usize;
        let expected_noncoding_deletions = (self.noncoding_length as f64 * self.noncoding_bp_deletion_prob) as usize;

        // Calculate window deletions
        let exon_windows = self.exon_length / self.window_size;
        let noncoding_windows = self.noncoding_length / self.window_size;
        
        let expected_exon_windows = (exon_windows as f64 * self.exon_window_deletion_prob) as usize;
        let expected_noncoding_windows = (noncoding_windows as f64 * self.noncoding_window_deletion_prob) as usize;
        
        let mut additional_exon_deletions = 0;
        let mut additional_noncoding_deletions = 0;
        
        // Process window deletions
        for _ in 0..expected_exon_windows {
            let deletion_size = rng.gen_range(1..=self.window_size);
            additional_exon_deletions += deletion_size;
        }
        
        for _ in 0..expected_noncoding_windows {
            let deletion_size = rng.gen_range(1..=self.window_size);
            additional_noncoding_deletions += deletion_size;
        }

        // Ensure we don't delete more than what exists
        let total_exon_deletions = (expected_exon_deletions + additional_exon_deletions)
            .min(self.exon_length);
        let total_noncoding_deletions = (expected_noncoding_deletions + additional_noncoding_deletions)
            .min(self.noncoding_length);

        (total_exon_deletions, total_noncoding_deletions)
    }

    pub fn run_simulation_round(&mut self) -> usize {
        self.simulation_round += 1;
        let mobilized = (self.active_te as f64 * self.te_mobilize_prob) as usize;
        self.te_mobilized += mobilized;

        // Generate TE lengths using normal distribution
        let normal = Normal::new(5000.0, 2000.0).unwrap();
        let te_lengths: Vec<usize> = (0..mobilized)
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::thread_rng();
                let length = normal.sample(&mut rng) as i32;
                length.clamp(100, 10000) as usize
            })
            .collect();

        let mut exon_insertions = 0;
        let mut noncoding_insertions = 0;
        let mut exon_growth = 0;
        let mut noncoding_growth = 0;

        // Process insertions
        for length in te_lengths {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(self.exon_length as f64 / self.genome_size as f64) {
                exon_insertions += 1;
                exon_growth += length;
            } else {
                noncoding_insertions += 1;
                noncoding_growth += length;
            }
        }

        self.te_in_exons += exon_insertions;
        self.te_in_noncoding += noncoding_insertions;

        // Process deletions
        let (exon_deletions, noncoding_deletions) = self.process_deletions();
        
        // Update genome metrics (use saturating arithmetic to prevent underflow)
        self.exon_length = (self.exon_length + exon_growth).saturating_sub(exon_deletions);
        self.noncoding_length = (self.noncoding_length + noncoding_growth).saturating_sub(noncoding_deletions);
        self.genome_size = self.exon_length + self.noncoding_length;
        self.bp_deleted_exon += exon_deletions;
        self.bp_deleted_noncoding += noncoding_deletions;

        exon_insertions
    }

    pub fn get_exon_ratio(&self) -> f64 {
        if self.genome_size == 0 {
            0.0
        } else {
            self.exon_length as f64 / self.genome_size as f64
        }
    }

    pub fn get_mutation_rate(&self) -> f64 {
        if self.te_mobilized == 0 {
            0.0
        } else {
            self.te_in_exons as f64 / self.te_mobilized as f64
        }
    }
}