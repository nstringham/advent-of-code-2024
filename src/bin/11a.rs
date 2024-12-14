use advent_of_code_2024::parsed_input_lines;

fn count_after_iterations(start: u64, iterations: usize) -> u64 {
    let digits = f64::log10(start as f64) as u64 + 1;

    if iterations == 0 {
        1
    } else if start == 0 {
        count_after_iterations(1, iterations - 1)
    } else if digits % 2 == 0 {
        let left = 10u64.pow(digits as u32 / 2);

        count_after_iterations(start / left, iterations - 1)
            + count_after_iterations(start % left, iterations - 1)
    } else {
        count_after_iterations(start * 2024, iterations - 1)
    }
}

fn main() {
    let input: Vec<u64> = parsed_input_lines()
        .nth(0)
        .expect("Input should not be empty");

    let answer: u64 = input
        .into_iter()
        .map(|start| count_after_iterations(start, 25))
        .sum();

    println!("{answer}");
}
