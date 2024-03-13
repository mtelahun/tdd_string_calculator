const SEPARATORS: [char; 2] = [',', '\n'];

pub fn add(arg: &str) -> Result<i32, Error> {
    if arg.is_empty() {
        return Ok(0);
    }

    let (separators, input) = get_input_and_separators(arg);
    let is_splitter = |digit| {
        if separators.contains(&digit) {
            return true;
        }

        false
    };
    let numbers: Vec<&str> = input.split(is_splitter).collect();
    if contains_trailing_separator(&numbers) {
        Err(Error::TrailingSeparator)
    } else {
        let mut total = 0;
        let mut negative_numbers = Vec::<i32>::new();
        for n in numbers {
            let number = match n.parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    let (invalid_char, position) =
                        get_invalid_character_and_position(n, separators[0]);
                    return Err(Error::InvalidSeparator(InvalidSeparator {
                        expected: separators[0],
                        actual: invalid_char,
                        position,
                    }));
                }
            };
            if number < 0 {
                negative_numbers.push(number);
            }
            total += number;
        }
        check_negative_numbers(&negative_numbers)?;
        Ok(total)
    }
}

fn get_input_and_separators(input: &str) -> (Vec<char>, &str) {
    let mut separator = Vec::new();
    let mut input_chars = input.chars();
    if input_chars.next().unwrap() == '/' && input_chars.next().unwrap() == '/' {
        let lines: Vec<&str> = input.split('\n').collect();
        separator.push(input_chars.next().unwrap());

        return (separator, lines[1]);
    }
    (SEPARATORS.to_vec(), input)
}

fn contains_trailing_separator(numbers: &[&str]) -> bool {
    numbers[numbers.len() - 1].is_empty()
}

fn get_invalid_character_and_position(input: &str, separator: char) -> (char, usize) {
    let mut position = 1;
    let mut invalid_char: char = separator;
    for c in input.chars().by_ref() {
        if !c.is_numeric() {
            invalid_char = c;
            break;
        }
        position += 1;
    }

    (invalid_char, position)
}

fn check_negative_numbers(numbers: &[i32]) -> Result<(), Error> {
    if !numbers.is_empty() {
        return Err(Error::NegativeNumber(NegativeNumber {
            numbers: numbers.to_vec(),
        }));
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum Error {
    TrailingSeparator,
    InvalidSeparator(InvalidSeparator),
    NegativeNumber(NegativeNumber),
}

#[derive(Debug, PartialEq)]
pub struct InvalidSeparator {
    expected: char,
    actual: char,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub struct NegativeNumber {
    numbers: Vec<i32>,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::TrailingSeparator => String::from("trailing separator in input"),
            Error::InvalidSeparator(e) => e.to_string(),
            Error::NegativeNumber(e) => e.to_string(),
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

fn main() {
    println!("Hello, world!");
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
        let op = "1,2\n3,";

        // Act
        let result = add(op);

        // Assert
        assert!(result.is_err(), "given: 1,2\\n3, result is an error");
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
}
