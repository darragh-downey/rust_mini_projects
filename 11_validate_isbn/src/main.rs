use std::fmt;
use std::str::FromStr;

// type Result<T> = std::result::Result<T, IsbnError>;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum IsbnError {
    LongInput,
    ShortInput,
    FailedChecksum,
    InvalidCharacter(usize, char),
}

impl fmt::Display for IsbnError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IsbnError::LongInput => write!(fmt, "isbn input too long"),
            IsbnError::ShortInput => write!(fmt, "isbn input too short"),
            IsbnError::FailedChecksum => write!(fmt, "failed isbn checksum calculation"),
            IsbnError::InvalidCharacter(x, y) => {
                write!(fmt, "ran into an invalid character {} at position {}", x, y)
            }
        }
    }
}

impl FromStr for Isbn {
    type Err = IsbnError; // TODO: replace with appropriate type
                          // Err should cover:
                          // input too long
                          // input too short
                          // failed checksum

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(15);

        for (i, c) in s.char_indices() {
            match c {
                '-' => continue,
                '0'..='9' => digits.push(c.to_digit(10).unwrap() as u8),
                _ => {
                    return Err(IsbnError::InvalidCharacter(i, c));
                }
            }
        }

        let n_digits = digits.len();

        if n_digits > 13 {
            return Err(IsbnError::LongInput);
        } else if n_digits < 13 {
            return Err(IsbnError::ShortInput);
        }

        if digits[12] != calculate_check_digit(&digits) {
            return Err(IsbnError::FailedChecksum);
        }

        Ok(Isbn {
            raw: s.to_string(),
            digits: vec![],
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    const WEIGHTS: [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
    // multiply by weights and sum the result
    // - multiply each number by pre-assigned weights which alternate between 1 and 3
    let weight_applied: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&x, &y)| x * y)
        .map(|subtotal| subtotal as u32)
        .sum();
    // reduce sum to single digit
    // - start with 10, then subtract the remainder of dividing the sum by 10
    let check_digit = 10 - (weight_applied % 10);

    match check_digit {
        10 => 0_u8,
        x => x as u8,
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({}) is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
