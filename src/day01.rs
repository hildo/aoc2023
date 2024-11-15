use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calibrate(input_file_name: &str) -> u64 {
    let mut return_value = 0;
    if let Ok(lines) = read_lines(input_file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            return_value = return_value + recover_value(&line);
        }
    }
    return_value
}

fn recover_value(input: &str) -> u64 {
    let numeric_str: Vec<char> = input.chars().filter(|c| c.is_numeric()).collect();
    let first_idx = 0;
    let last_idx = numeric_str.len() - 1;

    let mut value_as_string = String::with_capacity(2);
    value_as_string.push(numeric_str[first_idx]);
    value_as_string.push(numeric_str[last_idx]);

    let my_integer: Result<u64, _> = value_as_string.parse();
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
    fn test_recover_value() {
        assert_eq!(recover_value("1abc2"), 12);
        assert_eq!(recover_value("pqr3stu8vwx"), 38);
        assert_eq!(recover_value("a1b2c3d4e5f"), 15);
        assert_eq!(recover_value("treb7uchet"), 77);
    }

    #[test]
    fn test_large() {
        let value = calibrate(".\\src\\resources\\day01_input.txt");
        println!("{}", value);
    }

}