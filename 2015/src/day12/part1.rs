use anyhow::Result;
use aoc_lib::read_to_char;

pub fn solve() -> Result<i64> {
    let chars = read_to_char("input/day12.in")?;
    let mut parser = JsonParser::new(chars);
    Ok(parser.parse_and_sum())
}

struct JsonParser {
    chars: Vec<char>,
    pos: usize,
}

impl JsonParser {
    fn new(chars: Vec<char>) -> Self {
        Self { chars, pos: 0 }
    }

    fn parse_and_sum(&mut self) -> i64 {
        self.parse_value()
    }

    fn parse_value(&mut self) -> i64 {
        self.skip_whitespace();

        if self.pos > self.chars.len() {
            return 0;
        }

        match self.chars[self.pos] {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            '"' => self.parse_string(),
            '-' | '0'..='9' => self.parse_number(),
            _ => {
                self.pos += 1;
                0
            }
        }
    }

    fn parse_array(&mut self) -> i64 {
        self.pos += 1;
        let mut sum = 0;

        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == ']' {
            self.pos += 1;
            return 0;
        }

        loop {
            sum += self.parse_value();
            self.skip_whitespace();

            if self.pos >= self.chars.len() {
                break;
            }

            match self.chars[self.pos] {
                ',' => {
                    self.pos += 1;
                    continue;
                }
                ']' => {
                    self.pos += 1;
                    break;
                }
                _ => self.pos += 1,
            }
        }

        sum
    }

    fn parse_object(&mut self) -> i64 {
        self.pos += 1;
        let mut sum = 0;

        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == '}' {
            self.pos += 1;
            return 0;
        }

        loop {
            self.parse_value();
            self.skip_whitespace();

            if self.pos < self.chars.len() && self.chars[self.pos] == ':' {
                self.pos += 1;
            }

            sum += self.parse_value();
            self.skip_whitespace();

            if self.pos >= self.chars.len() {
                break;
            }

            match self.chars[self.pos] {
                ',' => {
                    self.pos += 1;
                    continue;
                }
                '}' => {
                    self.pos += 1;
                    break;
                }
                _ => self.pos += 1,
            }
        }

        sum
    }

    fn parse_number(&mut self) -> i64 {
        let start = self.pos;

        if self.pos < self.chars.len() && self.chars[self.pos] == '-' {
            self.pos += 1;
        }

        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        let number_str: String = self.chars[start..self.pos].iter().collect();
        number_str.parse::<i64>().unwrap_or(0)
    }

    fn parse_string(&mut self) -> i64 {
        self.pos += 1;

        while self.pos < self.chars.len() {
            match self.chars[self.pos] {
                '"' => {
                    self.pos += 1;
                    break;
                }
                '\\' => {
                    self.pos += 2;
                }
                _ => {
                    self.pos += 1;
                }
            }
        }

        0
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
}
