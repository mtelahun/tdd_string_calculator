const SEPARATORS: [char; 2] = [',', '\n'];

pub fn add(arg: &str) -> Result<i32, Error> {
    let is_splitter = |digit| {
        if SEPARATORS.contains(&digit) {
            return true;
        }

        return false;
    };
    let numbers: Vec<&str> = arg.split(is_splitter).collect();
    if arg.is_empty() {
        return Ok(0);
    } else if contains_trailing_separator(&numbers) {
        return Err(Error::TrailingSeparator);
    } else {
        let mut total = 0;
        for n in numbers {
            let number: i32 = n.parse().expect("failed to parse integer from string");
            total += number;
        }

        return Ok(total);
    }
}

fn contains_trailing_separator(numbers: &[&str]) -> bool {
    numbers[numbers.len() - 1].is_empty()
}

#[derive(Debug, PartialEq)]
pub enum Error {
    TrailingSeparator,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::TrailingSeparator => "trailing separator in input",
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for Error {}

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
}
