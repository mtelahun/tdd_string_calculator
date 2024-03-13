pub fn add(arg: &str) -> i32 {
    let is_splitter = |digit| {
        if digit == ',' || digit == '\n' {
            return true;
        }

        return false;
    };
    let numbers: Vec<&str> = arg.split(is_splitter).collect();
    if arg.is_empty() {
        return 0;
    } else {
        let mut total = 0;
        for n in numbers {
            let number: i32 = n.parse().expect("failed to parse integer from string");
            total += number;
        }

        return total;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn given_empty_string_return_0() {
        // Act
        let result = add("");

        // Assert
        assert_eq!(0, result, "given an empty string the result is 0")
    }

    #[test]
    fn given_only_1_number_return_the_number() {
        // Act
        let result = add("10");

        // Assert
        assert_eq!(
            10, result,
            "given only one number the result is the number itself"
        )
    }

    #[test]
    fn given_2_numbers_return_the_sum() {
        // Act
        let result = add("1,10");

        // Assert
        assert_eq!(11, result, "given \"1,10\" return 11")
    }

    #[test]
    fn given_3_numbers_return_the_sum() {
        // Act
        let result = add("1,1,10");

        // Assert
        assert_eq!(12, result, "given \"1,1,10\" return 12")
    }

    #[test]
    fn commas_or_newlines_may_act_as_separators() {
        // Arrange
        let op = "1,2\n3";

        // Act
        let result = add(op);

        // Assert
        assert_eq!(result, 6, "given: 1,2\\n3, result is 6");
    }
}
