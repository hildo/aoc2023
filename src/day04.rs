use crate::helpers;
use once_cell::sync::Lazy;
use regex::Regex;


fn load_cards(file_name: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    static CARD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Card.*:(?<winning>.*)\|(?<mine>.*)").unwrap());
    let mut ret_val: Vec<(Vec<u32>,Vec<u32>)> = Vec::new();

    if let Ok(lines) = helpers::read_lines(file_name) {
        for line in lines.flatten() {
            match CARD_RE.captures(&line) {
                Some(c) => {
                    let winning_numbers = &c["winning"];
                    let my_numbers = &c["mine"];
                    ret_val.push((convert_to_numeric_vector(winning_numbers), convert_to_numeric_vector(my_numbers)));
                },
                None => println!("No Match")
            }
        }
    }

    ret_val
}

fn convert_to_numeric_vector(input: &str) -> Vec<u32> {
    static NUMBERS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
    let this_value: Vec<u32> = NUMBERS_RE.find_iter(input).map(|value| value.as_str().parse().unwrap()).collect();
    this_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_simple() {
        let cards = load_cards("./src/resources/day04_simple.txt");
        let mut total_score = 0;
        for card in cards {
            total_score += 
                card.1.iter().filter(|number| card.0.contains(number)).fold(0, |acc, _| {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                });
        }
        assert_eq!(13, total_score);
    }

    #[test]
    fn part_one_() {
        let cards = load_cards("./src/resources/day04_input.txt");
        let mut total_score = 0;
        for card in cards {
            total_score += 
                card.1.iter().filter(|number| card.0.contains(number)).fold(0, |acc, _| {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                });
        }
        println!("{}", total_score);
        // assert_eq!(13, total_score);
    }

}
