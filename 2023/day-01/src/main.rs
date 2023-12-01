use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[rustfmt::skip]
const DIGITS: [&str; 19] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut result_part_1 = 0;
    let mut result_part_2 = 0;
    for line in reader.lines() {
        let line = line?;
        result_part_1 += get_calibration_value_1(&line);
        result_part_2 += get_calibration_value_2(&line);
    }

    println!(
        "[Part 1] The sum of calibration values is {}",
        result_part_1
    );
    println!(
        "[Part 2] The sum of calibration values is {}",
        result_part_2
    );

    Ok(())
}

fn get_calibration_value_1(s: &str) -> u32 {
    let first_digit = s
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();

    let last_digit = s
        .chars()
        .rfind(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();

    first_digit * 10 + last_digit
}

fn get_calibration_value_2(s: &str) -> u32 {
    println!("str: {s}");
    let first_finding = DIGITS
        .iter()
        .map(|&digit| (digit, s.find(digit)))
        .filter(|finding| finding.1.is_some())
        .min_by_key(|finding| finding.1.unwrap())
        .unwrap();
    println!("result {first_finding:?}");

    let last_finding = DIGITS
        .iter()
        .map(|&digit| (digit, s.rfind(digit)))
        .filter(|finding| finding.1.is_some())
        .max_by_key(|finding| finding.1.unwrap())
        .unwrap();
    println!("result {last_finding:?}");

    get_digit_value(first_finding.0) * 10 + get_digit_value(last_finding.0)
}

fn get_digit_value(digit: &str) -> u32 {
    let first_char = digit.chars().next().unwrap();
    if first_char.is_ascii_digit() {
        return first_char.to_digit(10).unwrap();
    }
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("unknown digit {digit}"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_calibration_value_1() {
        let td = HashMap::from([
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ]);
        for (test, expected) in td {
            assert_eq!(get_calibration_value_1(test), expected);
        }
    }

    #[test]
    fn test_get_calibration_value_2() {
        let td = HashMap::from([
            ("fivexmgsixsix282", 52),
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ]);
        for (test, expected) in td {
            assert_eq!(get_calibration_value_2(test), expected);
        }
    }
}
