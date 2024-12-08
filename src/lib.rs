use std::{
    env,
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
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
