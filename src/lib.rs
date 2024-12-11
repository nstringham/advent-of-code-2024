use std::{
    env,
    fmt::Debug,
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
    str::FromStr,
};

pub fn input_lines() -> Box<dyn Iterator<Item = String>> {
    match env::args().nth(1) {
        None => Box::new(lines_from_reader(stdin())),
        Some(filename) => Box::new(lines_from_reader(
            File::open(filename).expect("Error opening input file"),
        )),
    }
}

fn lines_from_reader<T: Read>(reader: T) -> impl Iterator<Item = String> {
    BufReader::new(reader)
        .lines()
        .map(|line| line.expect("Error reading input"))
}

pub fn parsed_input_lines<T>() -> impl Iterator<Item = Vec<T>>
where
    T: FromStr,
    T::Err: Debug,
{
    input_lines().map(|line| {
        line.split_whitespace()
            .map(|string| T::from_str(string).expect("Error Parsing Input"))
            .collect()
    })
}
