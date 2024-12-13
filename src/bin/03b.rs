use std::{iter::Peekable, str::Chars};

use advent_of_code_2024::input_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Do,
    Dont,
    Mul(u32, u32),
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

    fn parse_token(&mut self) -> Option<Token> {
        match self.inner.next()? {
            'm' => {
                self.inner.next_if_eq(&'u')?;
                self.inner.next_if_eq(&'l')?;
                self.inner.next_if_eq(&'(')?;
                let x = self.parse_number()?;
                self.inner.next_if_eq(&',')?;
                let y = self.parse_number()?;
                self.inner.next_if_eq(&')')?;

                Some(Token::Mul(x, y))
            }
            'd' => {
                self.inner.next_if_eq(&'o')?;

                if self.inner.next_if_eq(&'(').is_some() {
                    self.inner.next_if_eq(&')')?;

                    Some(Token::Do)
                } else if self.inner.next_if_eq(&'n').is_some() {
                    self.inner.next_if_eq(&'\'')?;
                    self.inner.next_if_eq(&'t')?;
                    self.inner.next_if_eq(&'(')?;
                    self.inner.next_if_eq(&')')?;

                    Some(Token::Dont)
                } else {
                    None
                }
            }
            _ => None,
        }
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
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mul) = self.parse_token() {
                return Some(mul);
            } else if self.inner.peek().is_none() {
                return None;
            }
        }
    }
}

fn main() {
    let mut total = 0;

    let mut doing = true;

    for line in input_lines() {
        for token in Parser::new(&line) {
            match token {
                Token::Do => doing = true,
                Token::Dont => doing = false,
                Token::Mul(x, y) => {
                    if doing {
                        total += x * y
                    }
                }
            }
        }
    }

    println!("{total}");
}
