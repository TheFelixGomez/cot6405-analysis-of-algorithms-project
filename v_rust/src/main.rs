use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

mod brute_force;
mod divide_and_conquer;

fn main() {
    let m: usize = 10;

    // Dictionaries to store the final average times, indexed by n
    let mut t_avg_alg1: HashMap<usize, f64> = HashMap::new();
    let mut t_avg_alg2: HashMap<usize, f64> = HashMap::new();

    // Get a handle to the thread-local random number generator
    let mut rng = rand::rng();

    // Define range constants
    let start_n: usize = 10_000;
    let end_n: usize = 55_000;
    let step_n: usize = 5_000;

    // for n in range(10**4, (55 * 10**3) + 1, 5 * 10**3):
    for n in (start_n..=end_n).step_by(step_n) {
        // print(f"Running tests for n = {n}...")
        println!("Running tests for n = {}...", n);

        let mut times_alg1_for_n: Vec<u128> = Vec::with_capacity(m);
        let mut times_alg2_for_n: Vec<u128> = Vec::with_capacity(m);

        for _ in 0..m {
            // Generate n distinct points
            let mut p: HashSet<(u16, u16)> = HashSet::with_capacity(n);

            while p.len() < n {
                let x: u16 = rng.random_range(0..=32_767);
                let y: u16 = rng.random_range(0..=32_767);
                p.insert((x, y));
            }

            // Convert set to list (knowing all points are distinct)
            let points: Vec<(f64, f64)> =
                p.into_iter().map(|(x, y)| (x as f64, y as f64)).collect();

            // compute ALG1:EmpiricalRT for the jth iteration
            let t1 = Instant::now();

            // call the function and assign the result to `_` to ignore it.
            // Assumes `points` is a Vec, so pass a slice `&points`.
            let _ = brute_force::brute_force_closest_points(&points);

            // .elapsed().as_nanos() gives the time difference as a u128.
            times_alg1_for_n.push(t1.elapsed().as_nanos());

            // compute ALG2:EmpiricalRT for the jth iteration
            let t1 = Instant::now();

            let _ = divide_and_conquer::divide_and_conquer_closest_points(&points);

            times_alg2_for_n.push(t1.elapsed().as_nanos());
        }

        // compute t_avg_ALG1...
        // .iter().sum() calculates the sum of the Vec
        let sum_alg1: u128 = times_alg1_for_n.iter().sum();
        t_avg_alg1.insert(n, sum_alg1 as f64 / m as f64);

        // compute t_avg_ALG2...
        let sum_alg2: u128 = times_alg2_for_n.iter().sum();
        t_avg_alg2.insert(n, sum_alg2 as f64 / m as f64);

        // Times are in nanoseconds, divide by 1_000_000.0 for milliseconds
        println!(
            "Average ALG1 time for n={}: {:.6} milliseconds",
            n,
            t_avg_alg1[&n] / 1_000_000.0
        );
        println!(
            "Average ALG2 time for n={}: {:.6} milliseconds\n",
            n,
            t_avg_alg2[&n] / 1_000_000.0
        );
    }

    println!("\n--- Final Average Runtimes (milliseconds) ---");
    println!("n\t|\tALG1 (Brute)\t|\tALG2 (D&C)");
    println!("{}", "-".repeat(100));

    // iterate over the original range to maintain insertion order,
    // as HashMap order is not guaranteed.
    for n_val in (start_n..=end_n).step_by(step_n) {
        println!(
            "{}\t|\t{:.6}\t|\t{:.6}",
            n_val,
            t_avg_alg1[&n_val] / 1_000_000.0,
            t_avg_alg2[&n_val] / 1_000_000.0
        );
    }
}

#[cfg(test)]
mod tests;
