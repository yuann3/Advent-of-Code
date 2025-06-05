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

enum JsonValue {
    Number(i64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
    Null,
    Bool,
}

impl JsonParser {
    fn new(chars: Vec<char>) -> Self {
        Self { chars, pos: 0 }
    }

    fn parse_and_sum(&mut self) -> i64 {
        let value = self.parse_value();
        self.sum_value(&value)
    }

    fn sum_value(&self, value: &JsonValue) -> i64 {
        match value {
            JsonValue::Number(n) => *n,
            JsonValue::Array(arr) => {
                arr.iter().map(|v| self.sum_value(v)).sum()
            },
            JsonValue::Object(obj) => {
                let has_red = obj.iter().any(|(_, v)| {
                    matches!(v, JsonValue::String(s) if s == "red")
                });

                if has_red {
                    0
                } else {
                    obj.iter().map(|(_, v)| self.sum_value(v)).sum()
                }
            },
            _ => 0,
        }
    }

    fn parse_value(&mut self) -> JsonValue {
        self.skip_whitespace();

        if self.pos >= self.chars.len() {
            return JsonValue::Null;
        }

        match self.chars[self.pos] {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            '"' => self.parse_string(),
            '-' | '0'..='9' => self.parse_number(),
            't' | 'f' => self.parse_bool(),
            'n' => self.parse_null(),
            _ => {
                self.pos += 1;
                JsonValue::Null
            }
        }
    }

    fn parse_object(&mut self) -> JsonValue {
        self.pos += 1; // Skip '{'
        let mut pairs = Vec::new();

        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == '}' {
            self.pos += 1;
            return JsonValue::Object(pairs);
        }

        loop {
            let key = match self.parse_value() {
                JsonValue::String(s) => s,
                _ => String::new(),
            };

            self.skip_whitespace();

            if self.pos < self.chars.len() && self.chars[self.pos] == ':' {
                self.pos += 1;
            }

            let value = self.parse_value();
            pairs.push((key, value));

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

        JsonValue::Object(pairs)
    }

    fn parse_array(&mut self) -> JsonValue {
        self.pos += 1; // Skip '['
        let mut elements = Vec::new();

        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == ']' {
            self.pos += 1;
            return JsonValue::Array(elements);
        }

        loop {
            elements.push(self.parse_value());
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

        JsonValue::Array(elements)
    }

    fn parse_string(&mut self) -> JsonValue {
        self.pos += 1;
        let start = self.pos;

        while self.pos < self.chars.len() {
            match self.chars[self.pos] {
                '"' => {
                    let content: String = self.chars[start..self.pos].iter().collect();
                    self.pos += 1;
                    return JsonValue::String(content);
                }
                '\\' => {
                    self.pos += 2;
                }
                _ => {
                    self.pos += 1;
                }
            }
        }

        JsonValue::String(String::new())
    }

    fn parse_number(&mut self) -> JsonValue {
        let start = self.pos;

        if self.pos < self.chars.len() && self.chars[self.pos] == '-' {
            self.pos += 1;
        }

        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        let number_str: String = self.chars[start..self.pos].iter().collect();
        let number = number_str.parse::<i64>().unwrap_or(0);
        JsonValue::Number(number)
    }

    fn parse_bool(&mut self) -> JsonValue {
        if self.chars[self.pos..].iter().take(4).collect::<String>() == "true" {
            self.pos += 4;
            JsonValue::Bool
        } else if self.chars[self.pos..].iter().take(5).collect::<String>() == "false" {
            self.pos += 5;
            JsonValue::Bool
        } else {
            self.pos += 1;
            JsonValue::Null
        }
    }

    fn parse_null(&mut self) -> JsonValue {
        self.pos += 4; // Skip "null"
        JsonValue::Null
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
}
