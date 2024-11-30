use crate::helpers;
use once_cell::sync::Lazy;
use regex::Regex;

fn load_schematic(file_name: &str) -> Vec<String> {
    let mut return_value: Vec<String> = Vec::new();
    if let Ok(lines) = helpers::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            return_value.push(line);
        }
    }
    return_value

}

fn load_part_numbers(schematic: Vec<String>) -> Vec<u64> {
    static NUMBER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
    static SYMBOL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\D)").unwrap());
    static FULL_STOP: &str = ".";

    let mut return_value: Vec<u64> = Vec::new();
    let mut idx = 0;
    while idx < schematic.len() {
        let row = &schematic[idx];
        for c in NUMBER_RE.captures_iter(row) {
            let unwrapped_match = c.get(0).unwrap();
            let (number, [_]) = c.extract();
            let start_idx = if unwrapped_match.start() > 0 {
                unwrapped_match.start() - 1
            } else {
                unwrapped_match.start()
            };
            let end_idx = if unwrapped_match.end() < row.len() - 1 {
                unwrapped_match.end() + 1
            } else {
                unwrapped_match.end()
            };
            
            let mut lines_to_check:Vec<&str> = Vec::new();
            if idx > 0 {
                lines_to_check.push(&schematic[idx-1][start_idx..end_idx]);
            }
            lines_to_check.push(&row[start_idx..end_idx]);
            if idx < schematic.len() - 1 {
                lines_to_check.push(&schematic[idx+1][start_idx..end_idx]);
            }

            for line in lines_to_check {
                let symbol_count = SYMBOL_RE.find_iter(line)
                    .filter(|value| !value.as_str().eq(FULL_STOP))
                    .count();
                if symbol_count > 0 {
                    match number.parse() {
                        Ok(numeric) => return_value.push(numeric),
                        Err(_) => println!("No number here")
                    }
                    break;
                }
            }
        }
        idx += 1;
    }
    return_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let schematic = load_schematic("./src/resources/day03_simple.txt");
        for line in schematic {
            println!("{}", line);
        }
    }

    #[test]
    fn test_part_numbers_simple() {
        let schematic = load_schematic("./src/resources/day03_simple.txt");
        let part_numbers = load_part_numbers(schematic);
        let sum = part_numbers.iter()
            .fold(0, |acc, part_number| acc + part_number);
        assert_eq!(4361, sum);
    }

    #[test]
    fn test_part_numbers() {
        let schematic = load_schematic("./src/resources/day03_input.txt");
        let part_numbers = load_part_numbers(schematic);
        let sum = part_numbers.iter()
            .fold(0, |acc, part_number| acc + part_number);
        println!("sum is {}", sum);
        assert_eq!(sum, 554003);
    }


}