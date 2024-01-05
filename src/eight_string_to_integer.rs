use std::num::{IntErrorKind, ParseIntError};

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim_start();

        // if first char is not a number, + or -, return 0
        let first_char = s.chars().next();
        if first_char.is_none() || (!first_char.unwrap().is_ascii_digit() && first_char.unwrap() != '-' && first_char.unwrap() != '+') {
            return 0;
        }

        let mut number_end = s[1..].find(|c: char| !c.is_ascii_digit());

        // Clip the number to i32::MAX and i32::MIN if it goes out of bounds
        let number = s[..(number_end.unwrap_or(s.len() - 1) + 1)].parse::<i64>().or_else(|e| match e.kind() {
            IntErrorKind::PosOverflow => Ok(i64::MAX),
            IntErrorKind::NegOverflow => Ok(i64::MIN),
            _ => Err(e),
        }).unwrap_or(0);

        return if number < i32::MIN as i64 {
            std::i32::MIN
        } else if number > i32::MAX as i64 {
            std::i32::MAX
        } else {
            number as i32
        }
    }
}