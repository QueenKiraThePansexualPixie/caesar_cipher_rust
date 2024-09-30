/// A 26-char array of the lowercase ASCII letters a-z.
const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Returns the index of the given `character` in the [`ASCII_LOWER`] array
/// constant, or `None` if the character is not in the list.
fn get_ascii_lower_index(character: char) -> Option<isize> {
    ASCII_LOWER
        .iter()
        .position(|&c| c == character)
        .map(|index| index as isize)
}

/// Returns the given index, shifted by the given `shift` value, looped to fit
/// within the possible indeces of [`ASCII_LOWER`].
fn shift_letter_index(index: isize, shift: isize) -> isize {
    let ascii_lower_length: isize = ASCII_LOWER.len() as isize;
    let shifted_index: isize = (index + shift) % ascii_lower_length;

    if shifted_index < 0 {
        ascii_lower_length - shifted_index.abs()
    } else {
        shifted_index
    }
}

/// Returns the given character, shifted by the given `shift` value for a
/// caesar cipher.
///
/// Does not account for numbers, punctuation, or any symbols other than ASCII
/// letters in the English alphabet.
///
/// See [shift_letter_index] and [loop_overflow] for more information.
fn caesar_shift_character(character: char, shift: isize) -> char {
    let [
            index_sanitise,
            return_sanitise,
        ]: [fn(&char) -> char; 2]
        = if character.is_ascii_uppercase() {
        [
            char::to_ascii_lowercase,
            char::to_ascii_uppercase,
        ]
    } else {
        [
            |c: &char| c.to_owned(),
            |c: &char| c.to_owned(),
        ]
    };

    let index: isize = match get_ascii_lower_index(index_sanitise(&character)) {
        Some(index) => index,
        None => return character,
    };

    let shifted_index: isize = shift_letter_index(index, shift);

    return_sanitise(&ASCII_LOWER[shifted_index as usize])
}

/// Returns the given text shifted by a specified `shift` value for a Caesar
/// Cipher.
///
/// # Examples
///
/// ```
/// use rust_caesar_cipher::caesar_cipher;
///
/// assert_eq!(caesar_cipher(1, "Hello world"), "Ifmmp xpsme");
/// ```
pub fn caesar_cipher(shift: isize, text: &str) -> String {
    let mut result = String::new();
    for character in text.chars() {
        result.push(caesar_shift_character(character, shift));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case('a', Some(0))]
    #[case('z', Some(25))]
    #[case('A', None)]
    #[case('1', None)]
    #[case('&', None)]
    fn test_get_ascii_lower_index(#[case] character: char, #[case] result: Option<isize>) {
        assert_eq!(get_ascii_lower_index(character), result);
    }

    #[rstest]
    #[case(5, 10, 15)]
    #[case(5, -10, 21)]
    fn test_shift_letter_index(#[case] index: isize, #[case] shift: isize, #[case] result: isize) {
        assert_eq!(shift_letter_index(index, shift), result);
    }

    #[rstest]
    #[case('a', 1, 'b')]
    #[case('A', 1, 'B')]
    #[case('a', 3, 'd')]
    #[case('A', 3, 'D')]
    #[case('a', -1, 'z')]
    #[case('A', -1, 'Z')]
    #[case('x', 3, 'a')]
    #[case('X', 3, 'A')]
    #[case('1', 1, '1')]
    ////#[case('1', 1, '2')]
    #[case('&', 3, '&')]
    fn test_caesar_shift_character(
        #[case] character: char,
        #[case] shift: isize,
        #[case] result: char,
    ) {
        assert_eq!(caesar_shift_character(character, shift), result);
    }

    #[rstest]
    #[case(0, "Hello, world!")]
    #[case(1, "Ifmmp, xpsme!")]
    #[case(-2, "Fcjjm, umpjb!")]
    fn test_caesar_cipher(#[case] shift: isize, #[case] result: &str) {
        assert_eq!(caesar_cipher(shift, "Hello, world!"), result.to_string());
    }
}
