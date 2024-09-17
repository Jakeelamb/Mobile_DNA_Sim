// Not using below, but keep for ideas maybe? 


// simulation.rs
// use rand::Rng;

// pub fn run_simulation() -> String {
//     let mut rng = rand::thread_rng();
//     let _genome_size = 1000000;
//     let te_mobilize_threshold = 0.5;
//     let mut exon_count = 0;
//     let mut non_coding_count = 0;

//     for _ in 0..1000 {
//         let mobilize = rng.gen::<f64>();
//         if mobilize < te_mobilize_threshold {
//             let prob_exon = rng.gen::<f64>();
//             if prob_exon < 0.5 {
//                 exon_count += 1;
//             } else {
//                 non_coding_count += 1;
//             }
//         }
//     }
    
//     format!(
//         "Simulation complete:\nExons: {}\nNon-coding regions: {}",
//         exon_count, non_coding_count
//     )
// }