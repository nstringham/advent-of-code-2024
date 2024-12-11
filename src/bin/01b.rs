use std::collections::HashMap;

use advent_of_code_2024::input_lines;

fn main() {
    let input = input_lines().map(|line| {
        let mut row = line.split_whitespace().map(|string| {
            string
                .parse::<u32>()
                .expect("Input should be a positive integer")
        });
        let left = row.next().expect("Input row should not be empty");
        let right = row.next().expect("Input row should have 2 values");
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
