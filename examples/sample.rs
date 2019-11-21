extern crate rand;
extern crate vose_alias_int;

use std::collections::{HashMap, BTreeMap};
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
use vose_alias_int::LookupTable;



fn main() {
    let mut p = [0_u64; 32];
    let mut total = 0;

    println!("Probabilities:");
    for i in 0 .. p.len() {
        p[i] = (i as u64) * 1000;
        println!("  {}: {}", i, p[i]);
        total += p[i];
    }
    println!("Total: {}", total);
    println!("");

    let alias = LookupTable::new(p);

    let mut rng = thread_rng();

    // Uniform distributions for x and y
    let dist_x = Uniform::new(0, alias.len());
    let dist_y = Uniform::new(0, alias.total());

    // Count sampling
    let mut samples: BTreeMap<usize, usize> = BTreeMap::new();
    const NUM_SAMPLES: usize = 1_000_000;
    for _ in 0 .. NUM_SAMPLES {
        let x = rng.sample(dist_x);
        let y = rng.sample(dist_y);
        let i = alias.sample(x, y);
        let mut n = samples.entry(i).or_default();
        *n += 1;
    }

    // Print samples
    println!("Sampling results ({} samples):", NUM_SAMPLES);
    for (i, n) in samples.iter() {
        println!("  {}: {} - {:.2} %", i, n, ((100 * n) as f32) / (NUM_SAMPLES as f32));
    }
}
