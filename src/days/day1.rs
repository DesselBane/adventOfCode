use crate::days::input::PUZZLE_1_INPUT;

pub fn puzzle1() -> Result<u32, &'static str> {
    PUZZLE_1_INPUT
        .lines()
        .map(extract_numbers_from_line)
        .try_fold(0, |acc, curr| Ok(acc + curr?))
}

fn extract_numbers_from_line(line: &str) -> Result<u32, &'static str> {
    let first_index = line
        .find(char::is_numeric)
        .ok_or("Bad input. Could not find first number")?;
    let last_index = line
        .rfind(char::is_numeric)
        .ok_or("Bad input. Could not find last number")?;

    let first = line.chars().nth(first_index).unwrap().to_digit(10).unwrap();
    let last = line.chars().nth(last_index).unwrap().to_digit(10).unwrap();

    Ok(first * 10 + last)
}

struct Match {
    value: &'static str,
    number_value: u32,
    current_index: usize,
}

impl Match {
    fn spawn_into(value: char, buf: &mut Vec<Match>) {
        match value {
            'o' => {
                buf.push(Match {
                    current_index: 0,
                    value: "one",
                    number_value: 1,
                });
            }
            't' => {
                buf.push(Match {
                    current_index: 0,
                    value: "two",
                    number_value: 2,
                });
                buf.push(Match {
                    current_index: 0,
                    value: "three",
                    number_value: 3,
                });
            }
            'f' => {
                buf.push(Match {
                    current_index: 0,
                    value: "four",
                    number_value: 4,
                });

                buf.push(Match {
                    current_index: 0,
                    value: "five",
                    number_value: 5,
                });
            }
            's' => {
                buf.push(Match {
                    current_index: 0,
                    value: "six",
                    number_value: 6,
                });
                buf.push(Match {
                    current_index: 0,
                    value: "seven",
                    number_value: 7,
                });
            }
            'e' => {
                buf.push(Match {
                    current_index: 0,
                    value: "eight",
                    number_value: 8,
                });
            }
            'n' => {
                buf.push(Match {
                    current_index: 0,
                    value: "nine",
                    number_value: 9,
                });
            }
            _ => {}
        }
    }

    fn rspawn_into(value: char, buf: &mut Vec<Match>) {
        match value {
            't' => {
                buf.push(Match {
                    current_index: 0,
                    value: "eight",
                    number_value: 8,
                });
            }
            'o' => {
                buf.push(Match {
                    current_index: 0,
                    value: "two",
                    number_value: 2,
                });
            }
            'e' => {
                buf.push(Match {
                    current_index: 0,
                    value: "one",
                    number_value: 1,
                });
                buf.push(Match {
                    current_index: 0,
                    value: "three",
                    number_value: 3,
                });
                buf.push(Match {
                    current_index: 0,
                    value: "five",
                    number_value: 5,
                });
                buf.push(Match {
                    current_index: 0,
                    value: "nine",
                    number_value: 9,
                });
            }
            'r' => {
                buf.push(Match {
                    current_index: 0,
                    value: "four",
                    number_value: 4,
                });
            }
            'x' => {
                buf.push(Match {
                    current_index: 0,
                    value: "six",
                    number_value: 6,
                });
            }
            'n' => {
                buf.push(Match {
                    current_index: 0,
                    value: "seven",
                    number_value: 7,
                });
            }
            _ => {}
        }
    }

    fn advance(mut self, value: char) -> Option<Self> {
        self.current_index += 1;

        match self.value.chars().nth(self.current_index) {
            None => None,
            Some(c) => {
                if c == value {
                    return Some(self);
                }
                None
            }
        }
    }

    fn radvance(mut self, value: char) -> Option<Self> {
        self.current_index += 1;

        match self.value.chars().rev().nth(self.current_index) {
            None => None,
            Some(c) => {
                if c == value {
                    return Some(self);
                }
                None
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.current_index + 1 >= self.value.len()
    }
}

pub fn puzzle2() -> Result<u32, &'static str> {
    PUZZLE_1_INPUT
        .lines()
        .map(extract_numbers_and_spelled_numbers_from_line)
        .try_fold(0, |acc, curr| Ok(acc + curr?))
}

fn extract_numbers_and_spelled_numbers_from_line(line: &str) -> Result<u32, &'static str> {
    let first = find_number(line).ok_or("Could not find first digit")?;

    let last = rfind_number(line).ok_or("Could not find last digit")?;

    Ok(first * 10 + last)
}

fn find_number(line: &str) -> Option<u32> {
    let mut matches: Vec<Match> = Vec::with_capacity(10);

    for c in line.chars() {
        // check for simple numbers
        if c.is_numeric() {
            return c.to_digit(10);
        }

        let mut local_matches: Vec<Match> = Vec::with_capacity(matches.capacity());

        // Advance all current matches
        while let Some(m) = matches.pop() {
            let advanced_match = m.advance(c);
            match advanced_match {
                None => continue,
                Some(new_match) => {
                    if new_match.is_complete() {
                        return Some(new_match.number_value);
                    }
                    local_matches.push(new_match)
                }
            }
        }

        //Spawn new matches
        Match::spawn_into(c, &mut local_matches);

        matches = local_matches;
    }

    None
}

fn rfind_number(line: &str) -> Option<u32> {
    let mut matches: Vec<Match> = Vec::with_capacity(10);

    for c in line.chars().rev() {
        // check for simple numbers
        if c.is_numeric() {
            return c.to_digit(10);
        }

        let mut local_matches: Vec<Match> = Vec::with_capacity(matches.capacity());

        // Advance all current matches
        while let Some(m) = matches.pop() {
            let advanced_match = m.radvance(c);
            match advanced_match {
                None => continue,
                Some(new_match) => {
                    if new_match.is_complete() {
                        return Some(new_match.number_value);
                    }
                    local_matches.push(new_match)
                }
            }
        }

        //Spawn new matches
        Match::rspawn_into(c, &mut local_matches);

        matches = local_matches;
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::days::day1::{
        extract_numbers_and_spelled_numbers_from_line, extract_numbers_from_line, Match,
    };
    use test_case::test_case;

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn it_extracts_numbers(line: &str, expected: u32) {
        let result = extract_numbers_from_line(line).unwrap();

        assert_eq!(expected, result)
    }

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    #[test_case("zoneight234", 14)]
    #[test_case("two1nine", 29)]
    #[test_case("eightwothree", 83)]
    #[test_case("abcone2threexyz", 13)]
    #[test_case("xtwone3four", 24)]
    #[test_case("4nineeightseven2", 42)]
    #[test_case("7pqrstsixteen", 76)]
    fn it_extracts_numbers_and_spelled_numbers(line: &str, expected: u32) {
        let result = extract_numbers_and_spelled_numbers_from_line(line).unwrap();

        assert_eq!(expected, result)
    }

    #[test]
    fn it_advances_and_completes() {
        let match_one = Match {
            current_index: 0,
            value: "one",
            number_value: 1,
        };

        let match_one = match_one.advance('n').unwrap();
        assert!(!match_one.is_complete());

        let match_one = match_one.advance('e').unwrap();
        assert!(match_one.is_complete());
    }
}
