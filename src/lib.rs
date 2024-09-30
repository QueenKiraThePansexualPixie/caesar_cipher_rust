const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn get_ascii_lower_index(character: char) -> Option<usize> {
    ASCII_LOWER.iter().position(|&c| c == character)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
enum size {
    isize(isize),
    usize(usize),
}

impl size {
    const fn to_isize(&self) -> isize {
        match self {
            Self::isize(n) => *n,
            Self::usize(n) => *n as isize,
        }
    }

    const fn to_usize(&self) -> usize {
        match self {
            Self::isize(n) => *n as usize,
            Self::usize(n) => *n,
        }
    }

    #[deprecated = "Not useful"]
    fn perform<T, F: FnOnce(Self) -> T>(self, performance: F) -> T {
        performance(self)
    }
}

impl std::ops::Add for size {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Self::isize(n1) => Self::isize(n1 + other.to_isize()),
            Self::usize(n1) => Self::usize(n1 + other.to_usize()),
        }
    }
}

impl std::ops::Sub for size {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Self::isize(n1) => Self::isize(n1 - other.to_isize()),
            Self::usize(n1) => Self::usize(n1 - other.to_usize()),
        }
    }
}

impl From<isize> for size {
    fn from(n: isize) -> Self {
        Self::isize(n)
    }
}

impl From<usize> for size {
    fn from(n: usize) -> Self {
        Self::usize(n)
    }
}

fn loop_overflow(number: size, max: size, min: size) -> size {
    match number {
        n if n > max => loop_overflow(n - max, max, min),
        n if n < min => loop_overflow(n + min, max, min),
        _ => number,
    }
}

fn shift_letter_index(index: size, shift: size) -> size {
    // TODO: account for when `shift` is negative and index is usize, so result 
    // TODO: is not bottomed out at 0
    loop_overflow(index + shift, 25usize.into(), 0usize.into())
}

// Does not account for numbers or punctuation.
fn caesar_shift_character(character: char, shift: isize) -> char {
    if character.is_ascii_uppercase() {
        let lower_character: char = character.to_ascii_lowercase();

        let index: usize = match get_ascii_lower_index(lower_character) {
            Some(index) => index,
            None => return character,
        };

        let shifted_index: usize = shift_letter_index(index.into(), shift.into()).to_usize();

        ASCII_LOWER[shifted_index].to_ascii_uppercase()
    } else {
        let index: usize = match get_ascii_lower_index(character) {
            Some(index) => index,
            None => return character,
        };

        let shifted_index: usize = shift_letter_index(index.into(), shift.into()).to_usize();

        ASCII_LOWER[shifted_index]
    }
}

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
    fn test_get_ascii_lower_index(#[case] character: char, #[case] result: Option<usize>) {
        assert_eq!(get_ascii_lower_index(character), result);
    }

    #[rstest]
    #[case(10isize.into(), 10isize.into(), 0isize.into(), 10isize.into())]
    #[case(0isize.into(), 10isize.into(), 0isize.into(), 0isize.into())]
    #[case(20isize.into(), 10isize.into(), 0isize.into(), 10isize.into())]
    #[case((-10isize).into(), 10isize.into(), 0isize.into(), 0isize.into())]
    fn test_remove_overflow_size(
        #[case] number: size,
        #[case] max: size,
        #[case] min: size,
        #[case] result: size,
    ) {
        assert_eq!(loop_overflow(number, max, min), result);
    }

    #[rstest]
    #[case(5usize.into(), 10isize.into(), 15usize.into())]
    #[case(5usize.into(), (-10isize).into(), 21usize.into())]
    fn test_shift_letter_index(#[case] index: size, #[case] shift: size, #[case] result: size) {
        assert_eq!(shift_letter_index(index, shift), result);
    }

    #[rstest]
    #[case('a', 1, 'b')]
    #[case('A', 1, 'B')]
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
