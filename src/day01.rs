use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static WORDS_TO_NUMBERS: &[(&str, &str)] = &[
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9")
];

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calibrate(input_file_name: &str) -> u128 {
    let mut return_value = 0;
    if let Ok(lines) = read_lines(input_file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("so far; {}", return_value);
            return_value = return_value + recover_value(&line);
        }
    }
    return_value
}

fn recover_value(input: &str) -> u128 {
    let mut numeric_str = String::new();
    let mut char_position = 0;

    while char_position < input.len() {
        let mut current_substr = &input[char_position..char_position+1];
        for entry in WORDS_TO_NUMBERS {
            let end_idx= char_position + entry.0.len();
            if end_idx <= input.len() {
                let sub_str = &input[char_position..end_idx];
                if sub_str == entry.0 {
                    current_substr = &entry.1;
                    break;
                }
            }
        }
        numeric_str.push_str(current_substr);
        char_position += 1;
    }

    let numeric_vec: Vec<char> = numeric_str.chars().filter(|c| c.is_numeric()).collect();
    let first_idx = 0;
    let last_idx = numeric_vec.len() - 1;

    let mut value_as_string = String::with_capacity(2);
    value_as_string.push(numeric_vec[first_idx]);
    value_as_string.push(numeric_vec[last_idx]);

    let my_integer: Result<u128, _> = value_as_string.parse();
    match my_integer {
        Ok(number) => number,
        Err(_) => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(calibrate(".\\src\\resources\\day01_simple.txt"), 142);
    }

    #[test]
    fn test_less_simple() {
        assert_eq!(calibrate(".\\src\\resources\\day01_less_simple.txt"), 281);
    }

    
    #[test]
    fn test_recover_value() {
        assert_eq!(recover_value("1abc2"), 12);
        assert_eq!(recover_value("pqr3stu8vwx"), 38);
        assert_eq!(recover_value("a1b2c3d4e5f"), 15);
        assert_eq!(recover_value("treb7uchet"), 77);
        assert_eq!(recover_value("two1nine"), 29);
        assert_eq!(recover_value("eightwothree"), 83);
        assert_eq!(recover_value("abcone2threexyz"), 13);
        assert_eq!(recover_value("xtwone3four"), 24);
        assert_eq!(recover_value("4nineeightseven2"), 42);
        assert_eq!(recover_value("zoneight234"), 14);
        assert_eq!(recover_value("7pqrstsixteen"), 76);
        assert_eq!(recover_value("eighthree"), 83);
        assert_eq!(recover_value("sevenine"), 79);
    }

    #[test]
    fn test_large() {
        let value = calibrate(".\\src\\resources\\day01_input.txt");
        println!("{}", value);
    }

}