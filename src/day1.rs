use std::{collections::HashMap, num::ParseIntError};

use crate::utils::{FromInput, Solution};

fn extract_calibration_value(s: &String) -> Result<u32, ParseIntError> {
    let digits: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
    let value = match (digits.first(), digits.last()) {
        (Some(f), Some(l)) => format!("{f}{l}").parse(),
        (Some(f), None) => format!("{f}{f}").parse(),
        _ => Ok(0),
    };
    return value;
}

fn replace_words(s: &String) -> String {
    let nums = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let mut result = s.clone();
    for (str, digit) in nums {
        let start = str.chars().nth(0).expect("Get first char");
        let end = str.chars().rev().nth(0).expect("Get last char");
        let sub = format!("{start}{digit}{end}");
        result = result.replace(str, &sub);
    }
    return result;
}

/// Model the problem for Day 1 using this struct
pub struct Day1 {
    lines: Vec<String>,
}

impl FromInput for Day1 {
    fn from_input(input: impl Iterator<Item = String>) -> Self {
        Day1 {
            lines: input.collect(),
        }
    }
}

impl Solution for Day1 {
    fn part_one(&self) -> String {
        let mut total = 0;
        for line in self.lines.iter() {
            if let Ok(val) = extract_calibration_value(line) {
                total += val
            }
        }
        return format!("{total}");
    }

    fn part_two(&self) -> String {
        let mut total = 0;
        for line in self.lines.iter() {
            let str = replace_words(line);
            if let Ok(val) = extract_calibration_value(&str) {
                total += val
            }
        }
        return format!("{total}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
        let result = replace_words(&String::from("twone"));
        assert_eq!(result, "21e");
    }

    #[test]
    fn test_part_one() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/1p1.txt"));
        let day = Day1::from_input(test_input);

        let result = day.part_one();
        assert_eq!(result, "142");
    }

    #[test]
    fn test_part_two() {
        use crate::load_input;
        let test_input = load_input(format!(".test_input/1p2.txt"));
        let day = Day1::from_input(test_input);

        let result = day.part_two();
        assert_eq!(result, "281");
    }
}
