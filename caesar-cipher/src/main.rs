use std::io;

#[cfg(test)]
mod tests {
    use crate::caesar_cipher;

    #[test]
    fn it_preserves_non_alphabetic_characters() {
        let input: &str = "?! 123 :\"`'";
        let shift_key: u32 = 3;

        assert_eq!(caesar_cipher(input, shift_key), "?! 123 :\"`'");
    }

    #[test]
    fn it_shifts_lowercase_characters_by_3() {
        let input: &str = "abcdefghijklmnopqrstuvwxyz";
        let shift_key: u32 = 3;

        assert_eq!(
            caesar_cipher(input, shift_key),
            "xyzabcdefghijklmnopqrstuvw"
        );
    }

    #[test]
    fn it_shifts_uppercase_characters_by_3() {
        let input: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let shift_key: u32 = 3;

        assert_eq!(
            caesar_cipher(input, shift_key),
            "XYZABCDEFGHIJKLMNOPQRSTUVW"
        );
    }

    #[test]
    fn it_shifts_uppercase_characters_by_5() {
        let input: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let shift_key: u32 = 5;

        assert_eq!(
            caesar_cipher(input, shift_key),
            "VWXYZABCDEFGHIJKLMNOPQRSTU"
        );
    }

    #[test]
    fn it_trims_the_input() {
        let input: &str = "\nabc\n";
        let shift_key: u32 = 3;

        assert_eq!(caesar_cipher(input, shift_key), "xyz");
    }
}

fn caesar_cipher<'life>(input: &'life str, shift_key: u32) -> String {
    let shift_chars = |c: char| -> char {
        let mut ascii_index = c as u32;

        if !c.is_alphabetic() {
            return c;
        }

        let shifted_ascii_index = ascii_index - shift_key;
        let overflows_uppercase_alphabet = shifted_ascii_index < 65;
        let overflows_lowercase_alphabet = shifted_ascii_index > 90 && ascii_index - shift_key < 97;

        if overflows_uppercase_alphabet || overflows_lowercase_alphabet {
            ascii_index += 26;
        }

        ascii_index -= shift_key;

        std::char::from_u32(ascii_index).unwrap_or(c)
    };

    return input.trim().chars().map(shift_chars).collect::<String>();
}

fn main() {
    println!("Enter a string to encode:");

    let mut string_to_encode = String::new();

    io::stdin()
        .read_line(&mut string_to_encode)
        .expect("Failed to read line");

    println!("Enter a shift key:");

    let mut shift_key = String::new();

    io::stdin()
        .read_line(&mut shift_key)
        .expect("Failed to read line");

    let encoded_string = caesar_cipher(&string_to_encode, shift_key.trim().parse::<u32>().unwrap());

    println!("Encoded string: {encoded_string}");
}
