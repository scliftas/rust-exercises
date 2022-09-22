use std::io;

#[cfg(test)]
mod tests {
    use crate::luhn_validator;

    #[test]
    fn valid_input_passes() {
        let input: &str = "79927398713";

        assert_eq!(luhn_validator(input), true);
    }

    #[test]
    fn invalid_input_fails() {
        let input: &str = "4147203059780942";

        assert_eq!(luhn_validator(input), false);
    }
}

fn str_to_digits<'life>(input: &'life str) -> impl DoubleEndedIterator<Item = u32> {
    input
        .trim()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
}

fn luhn_validator<'life>(input: &'life str) -> bool {
    let input_digits = str_to_digits(&input).rev();
    let mut calculated_digits: Vec<u32> = Vec::new();

    for (index, digit) in input_digits.enumerate() {
        if index % 2 == 0 {
            calculated_digits.push(digit.try_into().unwrap());
            continue;
        }

        let doubled = digit * 2;
        let doubled_digits = str_to_digits(&doubled.to_string());

        calculated_digits.push(doubled_digits.sum::<u32>());
    }

    return calculated_digits.into_iter().sum::<u32>() % 10 == 0;
}

fn main() {
    println!("Enter a number to validate:");

    let mut number_to_validate = String::new();

    io::stdin()
        .read_line(&mut number_to_validate)
        .expect("Failed to read line");

    let is_valid = luhn_validator(&number_to_validate);

    println!("Result: {is_valid}");
}
