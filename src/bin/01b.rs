use std::collections::HashMap;

use advent_of_code_2024::parsed_input_lines;

fn main() {
    let input = parsed_input_lines::<u32>().map(|row| {
        let left = row.get(0).expect("Input row should not be empty").clone();
        let right = row.get(1).expect("Input row should have 2 values").clone();
        (left, right)
    });

    let mut left_counts = HashMap::new();
    let mut right_counts = HashMap::new();
    for (left, right) in input {
        *left_counts.entry(left).or_insert(0) += 1;
        *right_counts.entry(right).or_insert(0) += 1;
    }

    let answer: u32 = left_counts
        .into_iter()
        .map(|(id, left_count)| id * left_count * right_counts.get(&id).cloned().unwrap_or(0))
        .sum();

    println!("{answer}");
}
