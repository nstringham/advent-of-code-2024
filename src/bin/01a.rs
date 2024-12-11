use advent_of_code_2024::input_lines;

fn main() {
    let input = input_lines().map(|line| {
        let mut row = line.split_whitespace().map(|string| {
            string
                .parse::<u32>()
                .expect("Input should be a positive integer")
        });
        let first = row.next().expect("Input row should not be empty");
        let second = row.next().expect("Input row should have 2 values");
        (first, second)
    });

    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    for (first, second) in input {
        first_list.push(first);
        second_list.push(second);
    }

    first_list.sort_unstable();
    second_list.sort_unstable();

    let answer: u32 = first_list
        .into_iter()
        .zip(second_list)
        .map(|(first, second)| first.abs_diff(second))
        .sum();

    println!("{answer}");
}
