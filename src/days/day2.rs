use crate::days::input::{PUZZLE_3_INPUT, PUZZLE_3_TEST_INPUT};

struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Default, Debug, Eq, PartialEq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new_multi(input: &str) -> Result<Vec<Game>, String> {
        input
            .lines()
            .map(Game::new)
            .try_fold(Vec::<Game>::new(), |mut acc, curr| {
                acc.push(curr?);
                Ok(acc)
            })
    }

    fn new(input: &str) -> Result<Game, String> {
        let (game_str, sets) = input
            .split_once(':')
            .ok_or(format!("Could not parse input line into Game: {}", input))?;

        let (_, game_str_id) = game_str
            .split_once(' ')
            .ok_or(format!("Could not split game id: {}.", game_str,))?;

        let game_id = game_str_id.parse::<u32>().map_err(|err| {
            format!(
                "Could not parse game id: {}. Got error: {}",
                game_str_id, err,
            )
        })?;

        let sets = Set::new_multi(sets)?;

        Ok(Game { id: game_id, sets })
    }

    fn possible_with(&self, input: &Set) -> bool {
        for set in self.sets.iter() {
            if !set.possible_with(input) {
                return false;
            }
        }
        true
    }
}

impl Set {
    fn new_multi(input: &str) -> Result<Vec<Set>, String> {
        input
            .split(';')
            .map(Set::new)
            .try_fold(Vec::<Set>::new(), |mut acc, curr| {
                acc.push(curr?);
                Ok(acc)
            })
    }

    fn new(input: &str) -> Result<Set, String> {
        let color_pairs = input.split(',');
        let mut result = Set::default();

        for pair in color_pairs {
            let (number_str, color) = pair
                .trim()
                .split_once(' ')
                .ok_or(format!("Could not parse set color pair: {}", pair))?;

            let number = number_str.parse::<u32>().map_err(|err| {
                format!(
                    "Could not parse set number into u32: {}. Got error {}",
                    number_str, err
                )
            })?;

            match color {
                "red" => {
                    result.red = number;
                }
                "green" => {
                    result.green = number;
                }
                "blue" => {
                    result.blue = number;
                }
                _ => return Err(format!("Could not determin color, got: {}", color)),
            }
        }

        Ok(result)
    }

    fn possible_with(&self, input: &Set) -> bool {
        self.red <= input.red && self.green <= input.green && self.blue <= input.blue
    }
}

fn puzzle_3_with_input(games_input: &str, loaded_set: &str) -> Result<u32, String> {
    let set = Set::new(loaded_set)?;

    Ok(Game::new_multi(games_input)?.iter().fold(0, |acc, curr| {
        if curr.possible_with(&set) {
            acc + curr.id
        } else {
            acc
        }
    }))
}

pub fn puzzle3() -> Result<u32, String> {
    puzzle_3_with_input(PUZZLE_3_INPUT, "12 red, 13 green, 14 blue")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::input::PUZZLE_3_TEST_INPUT;
    use test_case::test_case;

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1)]
    #[test_case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 2)]
    #[test_case(
        "Game 200: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        200
    )]
    fn it_parses_game_id(input: &str, expected_id: u32) {
        let result = Game::new(input).unwrap();

        assert_eq!(expected_id, result.id)
    }

    #[test_case("3 blue, 4 red", Set{red: 4, green: 0, blue: 3})]
    #[test_case("1 red, 2 green, 6 blue", Set{red: 1, green: 2, blue: 6})]
    #[test_case("1 red, 20 green, 6 blue", Set{red: 1, green: 20, blue: 6})]
    fn it_parses_single_set(input: &str, expected: Set) {
        let result = Set::new(input).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn it_parses_multi_set() {
        let result = Set::new_multi("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        let expected = vec![
            Set {
                red: 4,
                green: 0,
                blue: 3,
            },
            Set {
                red: 1,
                green: 2,
                blue: 6,
            },
            Set {
                red: 0,
                green: 2,
                blue: 0,
            },
        ];

        assert!(expected.iter().all(|x| result.contains(x)))
    }

    #[test]
    fn it_parses_game() {
        let result = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        let expected_sets = vec![
            Set {
                red: 4,
                green: 0,
                blue: 3,
            },
            Set {
                red: 1,
                green: 2,
                blue: 6,
            },
            Set {
                red: 0,
                green: 2,
                blue: 0,
            },
        ];

        assert!(expected_sets.iter().all(|x| result.sets.contains(x)));
        assert_eq!(1, result.id);
    }

    #[test_case("3 blue, 4 red", "1 blue", false)]
    #[test_case("3 blue, 4 red", "5 blue, 4 red", true)]
    #[test_case("1 green", "2 blue, 6 red, 5 green", true)]
    #[test_case("3 blue, 4 red", "1 green, 3 blue", false)]
    #[test_case("3 blue, 4 red", "3 blue, 4 red", true)]
    fn set_is_possible_with(first_input: &str, second_input: &str, shoud_be_possible: bool) {
        let first_set = Set::new(first_input).unwrap();
        let second_set = Set::new(second_input).unwrap();

        assert_eq!(shoud_be_possible, first_set.possible_with(&second_set))
    }

    #[test_case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "12 red, 13 green, 14 blue",
        true
    )]
    #[test_case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "12 red, 13 green, 14 blue",
        false
    )]
    fn game_is_possible_with(game_input: &str, set_input: &str, should_be_possible: bool) {
        let game = Game::new(game_input).unwrap();
        let set = Set::new(set_input).unwrap();

        assert_eq!(should_be_possible, game.possible_with(&set));
    }

    #[test]
    fn it_prduces_test_output() {
        let result = puzzle_3_with_input(PUZZLE_3_TEST_INPUT, "12 red, 13 green, 14 blue").unwrap();

        assert_eq!(8, result)
    }
}
