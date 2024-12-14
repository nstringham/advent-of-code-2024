use std::collections::HashMap;

use advent_of_code_2024::parsed_input_lines;

#[derive(Default)]
struct MemoizedCountAfterIterations {
    cache: HashMap<(u64, usize), u64>,
}

impl MemoizedCountAfterIterations {
    fn count_after_iterations(&mut self, start: u64, iterations: usize) -> u64 {
        if iterations == 0 {
            return 1;
        }

        let key = (start, iterations);

        if let Some(&cached) = self.cache.get(&key) {
            return cached;
        }

        let digits = f64::log10(start as f64) as u64 + 1;

        let value = if start == 0 {
            self.count_after_iterations(1, iterations - 1)
        } else if digits % 2 == 0 {
            let left = 10u64.pow(digits as u32 / 2);

            self.count_after_iterations(start / left, iterations - 1)
                + self.count_after_iterations(start % left, iterations - 1)
        } else {
            self.count_after_iterations(start * 2024, iterations - 1)
        };

        self.cache.insert(key, value);

        value
    }
}

fn main() {
    let input: Vec<u64> = parsed_input_lines()
        .nth(0)
        .expect("Input should not be empty");

    let mut cache = MemoizedCountAfterIterations::default();

    let answer: u64 = input
        .into_iter()
        .map(|start| cache.count_after_iterations(start, 75))
        .sum();

    println!("{answer}");
}
