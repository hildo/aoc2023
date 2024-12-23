use crate::helpers;
use once_cell::sync::Lazy;
use regex::Regex;

struct Sample {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    id: u32,
    samples: Vec<Sample>,
}

fn load_sample(input: &str) -> Vec<Sample> {
    static RED_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d+) red.*").unwrap());
    static GREEN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d+) green.*").unwrap());
    static BLUE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d+) blue.*").unwrap());

    let mut return_value: Vec<Sample> = Vec::new();

    let samples = input.split(";");
    for sample in samples {
        let red_value = match RED_RE.captures(&sample) {
            Some(val) => val["count"].parse().unwrap_or_default(),
            None => 0
        };
        let green_value = match GREEN_RE.captures(&sample) {
            Some(val) => val["count"].parse().unwrap_or_default(),
            None => 0
        };
        let blue_value = match BLUE_RE.captures(&sample) {
            Some(val) => val["count"].parse().unwrap_or_default(),
            None => 0
        };
        return_value.push(Sample {
            red: red_value,
            green: green_value,
            blue: blue_value
        });
    }
    return_value
}

fn load_games(file_name: &str) -> Vec<Game> {
    static GAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?<id>\d+): (?<samples>.*)").unwrap());
    let mut return_value = Vec::new();
    if let Ok(lines) = helpers::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            // println!("{}", line);
            let game = GAME_RE.captures(&line);
            match game {
                Some(details) => {
                    return_value.push(Game {
                        id: details["id"].parse().unwrap_or_default(),
                        samples: load_sample(&details["samples"])
                    });
                },
                None => unimplemented!()
            }
        }
    }
    return_value
}

fn id_sum_of_possible_games(configuration: &Sample, games: &Vec<Game>) -> u32 {
    games.iter()
        .filter(|game| {
            game.samples.iter()
                .map(|sample| sample.red <= configuration.red && sample.green <= configuration.green && sample.blue <= configuration.blue) 
                .all(|valid_sample| valid_sample == true)
        }).fold(0, |acc, game| acc + game.id)
}

fn min_configuration(game: &Game) -> Sample {
    game.samples.iter().fold(Sample{red: 0, green: 0, blue: 0}, |acc, sample| {
        let my_red = if sample.red > acc.red { sample.red } else { acc.red };
        let my_green = if sample.green > acc.green { sample.green } else { acc.green };
        let my_blue = if sample.blue > acc.blue { sample.blue } else { acc.blue };
        Sample{red: my_red, green: my_green, blue: my_blue}
    })
}

fn power_of_minimum_set(games: &Vec<Game>) -> u32 {
    games.iter()
        .map(|game| min_configuration(game))
        .fold(0, |acc, config| acc + config.red * config.green * config.blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_power_of_mins() {
        let games = load_games("./src/resources/day02_simple.txt");
        let value = power_of_minimum_set(&games);
        
        print!("Value is {}", value);
        assert_eq!(value, 2286);
    }

    #[test]
    fn power_of_mins() {
        let games = load_games("./src/resources/day02_input.txt");
        let value = power_of_minimum_set(&games);
        
        print!("Value is {}", value);
    }

    #[test]
    fn sample_is_possible() {
        let configuration= Sample {
            red: 12, 
            green: 13,
            blue: 14
        };
        let games = load_games("./src/resources/day02_simple.txt");
        let sum = id_sum_of_possible_games(&configuration, &games);
        
        print!("Sum is {}", sum);
        assert_eq!(sum, 8);
    }

    #[test]
    fn is_possible() {
        let configuration= Sample {
            red: 12, 
            green: 13,
            blue: 14
        };
        let games = load_games("./src/resources/day02_input.txt");
        let sum = id_sum_of_possible_games(&configuration, &games);
        
        print!("Sum is {}", sum);
    }

    #[test]
    fn test_load_games() {
        let games = load_games("./src/resources/day02_simple.txt");
        assert_eq!(games.len(), 5);

        // Game 1
        assert_eq!(games[0].id, 1);
        assert_eq!(games[0].samples.len(), 3);
        assert_eq!(games[0].samples[0].blue, 3);
        assert_eq!(games[0].samples[0].red, 4);
        assert_eq!(games[0].samples[0].green, 0);

        assert_eq!(games[0].samples[1].blue, 6);
        assert_eq!(games[0].samples[1].red, 1);
        assert_eq!(games[0].samples[1].green, 2);

        assert_eq!(games[0].samples[2].blue, 0);
        assert_eq!(games[0].samples[2].red, 0);
        assert_eq!(games[0].samples[2].green, 2);

        // Game 2
        assert_eq!(games[1].id, 2);
        assert_eq!(games[1].samples.len(), 3);
        assert_eq!(games[1].samples[0].blue, 1);
        assert_eq!(games[1].samples[0].red, 0);
        assert_eq!(games[1].samples[0].green, 2);

        assert_eq!(games[1].samples[1].blue, 4);
        assert_eq!(games[1].samples[1].red, 1);
        assert_eq!(games[1].samples[1].green, 3);

        assert_eq!(games[1].samples[2].blue, 1);
        assert_eq!(games[1].samples[2].red, 0);
        assert_eq!(games[1].samples[2].green, 1);

        // Game 3
        assert_eq!(games[2].id, 3);
        assert_eq!(games[2].samples.len(), 3);
        assert_eq!(games[2].samples[0].blue, 6);
        assert_eq!(games[2].samples[0].red, 20);
        assert_eq!(games[2].samples[0].green, 8);

        assert_eq!(games[2].samples[1].blue, 5);
        assert_eq!(games[2].samples[1].red, 4);
        assert_eq!(games[2].samples[1].green, 13);

        assert_eq!(games[2].samples[2].blue, 0);
        assert_eq!(games[2].samples[2].red, 1);
        assert_eq!(games[2].samples[2].green, 5);

        // Game 4
        assert_eq!(games[3].id, 4);
        assert_eq!(games[3].samples.len(), 3);
        assert_eq!(games[3].samples[0].blue, 6);
        assert_eq!(games[3].samples[0].red, 3);
        assert_eq!(games[3].samples[0].green, 1);

        assert_eq!(games[3].samples[1].blue, 0);
        assert_eq!(games[3].samples[1].red, 6);
        assert_eq!(games[3].samples[1].green, 3);

        assert_eq!(games[3].samples[2].blue, 15);
        assert_eq!(games[3].samples[2].red, 14);
        assert_eq!(games[3].samples[2].green, 3);

        // Game 5
        assert_eq!(games[4].id, 5);
        assert_eq!(games[4].samples.len(), 2);
        assert_eq!(games[4].samples[0].blue, 1);
        assert_eq!(games[4].samples[0].red, 6);
        assert_eq!(games[4].samples[0].green, 3);

        assert_eq!(games[4].samples[1].blue, 2);
        assert_eq!(games[4].samples[1].red, 1);
        assert_eq!(games[4].samples[1].green, 2);
    }

    #[test]
    fn test_load_sample() {
        let samples = load_sample("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(samples.len(), 3);
        assert_eq!(samples[0].blue, 3);
        assert_eq!(samples[0].red, 4);
        assert_eq!(samples[0].green, 0);

        assert_eq!(samples[1].blue, 6);
        assert_eq!(samples[1].red, 1);
        assert_eq!(samples[1].green, 2);

        assert_eq!(samples[2].blue, 0);
        assert_eq!(samples[2].red, 0);
        assert_eq!(samples[2].green, 2);
    }


}