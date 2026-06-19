use std::hint::black_box;
use std::time::Instant;

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn expensive_operation(x: &f64) -> f64 {
    // Wrapping as black box so it doesn't get optimized away
    black_box(x.sin().powi(2) + x.cos().powi(2))
}

fn main() {
    let num_elements = 50_000_000;
    let data: Vec<f64> = (0..num_elements)
        .into_par_iter() // Using rayon in setup because otherwise it gets slow
        .map(|x| x as f64)
        .collect();

    let start = Instant::now();
    let _: f64 = data.iter().map(expensive_operation).sum();
    let serial_duration = start.elapsed();
    println!("Serial Duration: {:?}", serial_duration);

    println!("Rayon threads: {}", rayon::current_num_threads());

    let start = Instant::now();
    let _: f64 = data.par_iter().map(expensive_operation).sum();
    let parallel_duration = start.elapsed();
    println!("Parallel Duration: {:?}", parallel_duration);

    println!(
        "Speedup: {:.2}x",
        serial_duration.as_secs_f64() / parallel_duration.as_secs_f64()
    );
}
