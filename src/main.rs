use rand::Rng;
use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let group_size: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(12);

    let num_trials: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50000);

    let mut rng = rand::thread_rng();
    let mut duplicate_count = 0;

    for _ in 0..num_trials {
        let birthdays: Vec<u32> = (0..group_size).map(|_| rng.gen_range(1..=365)).collect();

        let unique: HashSet<u32> = HashSet::from_iter(birthdays.into_iter());

        if unique.len() < group_size {
            duplicate_count += 1;
        }
    }

    let probability = (duplicate_count as f64 / num_trials as f64) * 100.0;
    println!(
        "After {} trials with {} people:\nDuplicate probability = {:.2}%",
        num_trials, group_size, probability
    );
}
