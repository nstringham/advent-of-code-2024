use std::{iter::Peekable, str::Chars};

use advent_of_code_2024::input_lines;

struct Mul(u32, u32);

impl Mul {
    pub fn product(&self) -> u32 {
        self.0 * self.1
    }
}

struct Parser<'a> {
    inner: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: input.chars().peekable(),
        }
    }

    fn parse_mul(&mut self) -> Option<Mul> {
        let m = self.inner.next()?;
        if m != 'm' {
            return None;
        }
        self.inner.next_if_eq(&'u')?;
        self.inner.next_if_eq(&'l')?;
        self.inner.next_if_eq(&'(')?;
        let x = self.parse_number()?;
        self.inner.next_if_eq(&',')?;
        let y = self.parse_number()?;
        self.inner.next_if_eq(&')')?;

        Some(Mul(x, y))
    }

    fn parse_number(&mut self) -> Option<u32> {
        fn is_digit(c: &char) -> bool {
            matches!(c, '0'..='9')
        }

        let mut number = self.inner.next_if(is_digit)? as u32 - '0' as u32;

        for _ in 0..2 {
            if let Some(digit) = self.inner.next_if(is_digit) {
                number *= 10;
                number += digit as u32 - '0' as u32
            }
        }

        Some(number)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Mul;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mul) = self.parse_mul() {
                return Some(mul);
            } else if self.inner.peek().is_none() {
                return None;
            }
        }
    }
}

fn main() {
    let input = input_lines();

    let answer: u32 = input
        .map(|line| Parser::new(&line).map(|mul| mul.product()).sum::<u32>())
        .sum();

    println!("{answer}");
}
