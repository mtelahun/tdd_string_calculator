use std::num::ParseIntError;

const SEPARATORS: [char; 2] = [',', '\n'];

pub trait StringCalculatorError {}

pub fn add(arg: &str) -> Result<i32, Error> {
    if arg.is_empty() {
        return Ok(0);
    }

    let (input, separators) = get_input_and_separators(arg);

    let (total, error_list) = parse_and_sum(input, &separators);

    return_ok_or_err(error_list, total)
}

fn get_input_and_separators(input: &str) -> (&str, Vec<char>) {
    let mut separator = Vec::new();
    let mut input_chars = input.chars();
    if input_chars.next().unwrap() == '/' && input_chars.next().unwrap() == '/' {
        let lines: Vec<&str> = input.split('\n').collect();
        separator.push(input_chars.next().unwrap());

        return (lines[1], separator);
    }
    (input, SEPARATORS.to_vec())
}

fn parse_and_sum(input: &str, separators: &[char]) -> (i32, MultipleErrorKind) {
    let is_splitter = |digit| {
        if separators.contains(&digit) {
            return true;
        }

        false
    };
    let numbers: Vec<&str> = input.split(is_splitter).collect();
    let mut multi_error = MultipleErrorKind { errors: Vec::new() };
    if contains_trailing_separator(&numbers) {
        multi_error.errors.push(Error::TrailingSeparator);
        return (0, multi_error);
    }

    let mut negative_numbers = Vec::new();
    let mut positive_numbers = Vec::new();
    match parse_numbers(input, separators) {
        Ok((mut pos_nums, mut neg_nums)) => {
            negative_numbers.append(&mut neg_nums);
            positive_numbers.append(&mut pos_nums)
        }
        Err(_) => {
            let (invalid_char, position) = get_invalid_character_and_position(input, separators[0]);
            multi_error
                .errors
                .push(Error::InvalidSeparator(InvalidSeparator {
                    expected: separators[0],
                    actual: invalid_char,
                    position,
                }));

            // We are in error state, but still need to check for negative numbers; use default separators
            let (_, mut neg_nums) =
                parse_numbers(input, &SEPARATORS).unwrap_or((Vec::new(), Vec::new()));
            negative_numbers.append(&mut neg_nums);
        }
    }
    let total = positive_numbers.iter().sum::<i32>();
    match no_negative_numbers_or_err(&negative_numbers) {
        Ok(_) => (),
        Err(e) => multi_error.errors.push(e),
    };

    (total, multi_error)
}

fn contains_trailing_separator(numbers: &[&str]) -> bool {
    numbers[numbers.len() - 1].is_empty()
}

fn get_invalid_character_and_position(input: &str, separator: char) -> (char, usize) {
    let mut position = 1;
    let mut invalid_char: char = separator;
    for c in input.chars().by_ref() {
        if !c.is_numeric() && c != separator {
            invalid_char = c;
            break;
        }
        position += 1;
    }

    (invalid_char, position)
}

fn parse_numbers(input: &str, separators: &[char]) -> Result<(Vec<i32>, Vec<i32>), ParseIntError> {
    let mut negative_numbers = Vec::new();
    let mut positive_numbers = Vec::new();
    let is_splitter = |digit| {
        if separators.contains(&digit) {
            return true;
        }

        false
    };

    let mut error = Vec::<ParseIntError>::new();
    let parts: Vec<&str> = input.split(is_splitter).collect();
    for part in parts {
        let num = match part.parse::<i32>() {
            Ok(n) => n,
            Err(e) => {
                error.push(e);

                // we're in an error state, setting the number to zero causes the least side-effects
                0
            }
        };
        if num < 0 {
            negative_numbers.push(num);
        } else {
            positive_numbers.push(num);
        }
    }

    if error.is_empty() || (!error.is_empty() && !negative_numbers.is_empty()) {
        Ok((positive_numbers, negative_numbers))
    } else {
        Err(error[0].clone())
    }
}

fn no_negative_numbers_or_err(numbers: &[i32]) -> Result<(), Error> {
    if !numbers.is_empty() {
        return Err(Error::NegativeNumber(NegativeNumber {
            numbers: numbers.to_vec(),
        }));
    }

    Ok(())
}

fn return_ok_or_err(multi_error: MultipleErrorKind, total: i32) -> Result<i32, Error> {
    if !multi_error.errors.is_empty() {
        Err(return_error(multi_error))
    } else {
        Ok(total)
    }
}

fn return_error(multi_error: MultipleErrorKind) -> Error {
    if multi_error.errors.len() > 1 {
        Error::Multiple(multi_error)
    } else {
        multi_error.errors[0].clone()
    }
}

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    TrailingSeparator,
    InvalidSeparator(InvalidSeparator),
    NegativeNumber(NegativeNumber),
    Multiple(MultipleErrorKind),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidSeparator {
    expected: char,
    actual: char,
    position: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NegativeNumber {
    numbers: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MultipleErrorKind {
    errors: Vec<Error>,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::TrailingSeparator => String::from("trailing separator in input"),
            Error::InvalidSeparator(e) => e.to_string(),
            Error::NegativeNumber(e) => e.to_string(),
            Error::Multiple(_) => String::from("multiple errors"),
        };

        write!(f, "{msg}")
    }
}

impl std::fmt::Display for InvalidSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{}' expected, but found '{}' at position {}",
            self.expected, self.actual, self.position
        )
    }
}

impl std::fmt::Display for NegativeNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut as_string = Vec::<String>::new();
        for n in self.numbers.iter() {
            as_string.push(format!("{n}"))
        }
        write!(
            f,
            "Negative number(s) are not allowed: {:?}",
            as_string.join(",")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{add, Error};

    #[test]
    fn given_empty_string_return_0() {
        // Act
        let result = add("");

        // Assert
        assert_eq!(0, result.unwrap(), "given an empty string the result is 0")
    }

    #[test]
    fn given_only_1_number_return_the_number() {
        // Act
        let result = add("10");

        // Assert
        assert_eq!(
            10,
            result.unwrap(),
            "given only one number the result is the number itself"
        )
    }

    #[test]
    fn given_2_numbers_return_the_sum() {
        // Act
        let result = add("1,10");

        // Assert
        assert_eq!(11, result.unwrap(), "given \"1,10\" return 11")
    }

    #[test]
    fn given_3_numbers_return_the_sum() {
        // Act
        let result = add("1,1,10");

        // Assert
        assert_eq!(12, result.unwrap(), "given \"1,1,10\" return 12")
    }

    #[test]
    fn commas_or_newlines_may_act_as_separators() {
        // Arrange
        let op = "1,2\n3";

        // Act
        let result = add(op);

        // Assert
        assert_eq!(result.unwrap(), 6, "given: 1,2\\n3 result is 6");
    }

    #[test]
    fn given_calculation_with_trailing_comma_return_error() {
        // Arrange
        let op = "1,2,";

        // Act
        let result = add(op);

        // Assert
        assert!(result.is_err(), "given: 1,2, result is an error");
        assert_eq!(
            result.unwrap_err(),
            Error::TrailingSeparator,
            "given: 1,2\\n3, TrailingSeparator Error is returned"
        );
    }

    #[test]
    fn given_initial_line_to_set_separator_then_use_new_separator() {
        // Arrange
        let op = "//|\n1|3";

        // Act
        let result = add(op);

        // Assert
        assert_eq!(result.unwrap(), 4, "given: //|\n1|3 result is 4");
    }

    #[test]
    fn given_a_specified_separator_when_another_separator_is_used_then_error() {
        // Arrange
        let op = "//;\n1|2";

        // Act
        let result = add(op);

        // Assert
        assert!(result.is_err(), "given: //;\n1|2 result is an error");
        assert_eq!(
            result.err().unwrap(),
            Error::InvalidSeparator(crate::InvalidSeparator {
                expected: ';',
                actual: '|',
                position: 2
            })
        );
    }

    #[test]
    fn given_a_negative_number_then_error() {
        // Arrange
        let op = "1,-2";

        // Act
        let result = add(op);

        // Assert
        assert!(result.is_err(), "given: 1,-2 result is an error");
        assert_eq!(
            result.err().unwrap(),
            Error::NegativeNumber(crate::NegativeNumber { numbers: vec![-2] })
        );
    }

    #[test]
    fn given_multiple_negative_numbers_then_error() {
        // Arrange
        let op = "2,-4,-9";

        // Act
        let result = add(op);

        // Assert
        assert!(
            result.is_err(),
            "given: 2,-4,-9 result is an error with all numbers"
        );
        assert_eq!(
            result.err().unwrap(),
            Error::NegativeNumber(crate::NegativeNumber {
                numbers: vec![-4, -9],
            })
        );
    }

    #[test]
    fn given_multiple_errors_then_return_all_errors() {
        // Arrange
        let op = "//|\n1|2,-3";

        // Act
        let result = add(op);

        // Assert
        assert!(
            result.is_err(),
            "given://|\n1|2,-3 result is multiple errors"
        );
        assert_eq!(
            result.err().unwrap(),
            Error::Multiple(crate::MultipleErrorKind {
                errors: vec![
                    Error::InvalidSeparator(crate::InvalidSeparator {
                        expected: '|',
                        actual: ',',
                        position: 4,
                    }),
                    Error::NegativeNumber(crate::NegativeNumber { numbers: vec![-3] })
                ],
            })
        );
    }
}
