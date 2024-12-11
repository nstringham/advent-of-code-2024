use advent_of_code_2024::parsed_input_lines;

fn main() {
    let input = parsed_input_lines::<u32>().map(|row| {
        let first = row.get(0).expect("Input row should not be empty").clone();
        let second = row.get(1).expect("Input row should have 2 values").clone();
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
