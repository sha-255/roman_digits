use std::fmt;
use std::str::FromStr;

pub struct RomanNumber {
    value: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRomanNumberError;

impl RomanNumber {
    pub fn to_i32(self) -> i32 {
        roman_to_int(self.value)
    }
}

impl From<&str> for RomanNumber {
    fn from(value: &str) -> Self {
        Self {
            value: String::from(value)
        }
    }
}

impl From<String> for RomanNumber {
    fn from(value: String) -> Self {
        Self {
            value
        }
    }
}

impl fmt::Debug for RomanNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for RomanNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for RomanNumber {
    type Err = ParseRomanNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RomanNumber::from(s))
    }
}

fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    for roman_digit in s.chars().rev().collect::<String>().to_ascii_uppercase().as_bytes() {
        let config_digit = number_of_roman_digit(roman_digit);
        if let Some(digit) = config_digit {
            if 4 * digit <= result {
                result -= digit;
            } else {
                result += digit;
            }
        } else {
            panic!("Incorrect roman number")
        }
    };
    result
}

fn number_of_roman_digit(digit: &u8) -> Option<i32> {
    match digit {
        b'I' => Some(1),
        b'V' => Some(5),
        b'X' => Some(10),
        b'L' => Some(50),
        b'C' => Some(100),
        b'D' => Some(500),
        b'M' => Some(1000),
        _ => None,
    }
}
