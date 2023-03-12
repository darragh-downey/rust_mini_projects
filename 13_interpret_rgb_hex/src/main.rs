use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum RgbError {
    InvalidRgb,
    FailedParseR,
    FailedParseG,
    FailedParseB,
    InvalidCharacter(usize, char),
}

impl fmt::Display for RgbError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // IsbnError::LongInput => write!(fmt, "isbn input too long"),
            RgbError::InvalidRgb => write!(fmt, "invalid rgb"),
            RgbError::FailedParseR => write!(fmt, "failed to parse r byte"),
            RgbError::FailedParseG => write!(fmt, "failed to parse g byte"),
            RgbError::FailedParseB => write!(fmt, "failed to parse b byte"),
            RgbError::InvalidCharacter(x, y) => {
                write!(fmt, "ran into an invalid character {} at position {}", x, y)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

impl FromStr for Rgb {
    type Err = RgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.trim().strip_prefix('#') {
            let r = u8::from_str_radix(&hex[0..2], 16);
            let g = u8::from_str_radix(&hex[2..4], 16);
            let b = u8::from_str_radix(&hex[4..6], 16);

            Ok(Rgb {
                r: r.or_else(|_err| Err(RgbError::FailedParseR))?,
                g: g.or_else(|_err| Err(RgbError::FailedParseG))?,
                b: b.or_else(|_err| Err(RgbError::FailedParseB))?,
            })
        } else {
            return Err(RgbError::InvalidRgb);
        }
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        // println!("{}", hex);
        let color: Rgb = hex.parse().unwrap();
        println!("{}", color);
    }
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
