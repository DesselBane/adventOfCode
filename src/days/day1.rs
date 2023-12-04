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

#[cfg(test)]
mod tests {
    use crate::days::day1::extract_numbers_from_line;
    use test_case::test_case;

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn it_extracts_numbers(line: &str, expected: u32) {
        let result = extract_numbers_from_line(line).unwrap();

        assert_eq!(expected, result)
    }
}
