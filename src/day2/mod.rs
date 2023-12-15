use std::cmp::max;

use regex::{Captures, Regex};

use crate::util::read_input;

struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

impl Round {
    fn empty() -> Round {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

struct Game {
    id: u8,
    rounds: Vec<Round>,
}

impl Game {
    fn empty() -> Game {
        Game {
            id: 0,
            rounds: vec![],
        }
    }

    fn is_invalid(&self) -> bool {
        self.rounds
            .iter()
            .any(|r| r.red > 12 || r.green > 13 || r.blue > 14)
    }

    fn min_set(&self) -> Round {
        self.rounds.iter().fold(Round::empty(), |acc, r| Round {
            red: max(acc.red, r.red),
            green: max(acc.green, r.green),
            blue: max(acc.blue, r.blue),
        })
    }

    fn power(&self) -> u32 {
        let set = self.min_set();
        (set.red as u32) * (set.green as u32) * (set.blue as u32)
    }
}

fn extract_capture(c: &Captures, name: &str) -> u8 {
    c.name(name)
        .and_then(|m| Some(m.as_str()))
        .unwrap_or("0")
        .parse::<u8>()
        .unwrap()
}

fn parse_round(round_str: &str) -> Round {
    let re = Regex::new(r"(?:(?P<red>\d+) red)|(?:(?P<green>\d+) green)|(?:(?P<blue>\d+) blue)")
        .unwrap();
    re.captures_iter(round_str)
        .fold(Round::empty(), |acc, c| Round {
            red: acc.red + extract_capture(&c, "red"),
            green: acc.green + extract_capture(&c, "green"),
            blue: acc.blue + extract_capture(&c, "blue"),
        })
}

fn parse_game(line: &str) -> Game {
    let re = Regex::new(r"Game (?P<game_id>\d+): ").unwrap();
    let match_option = re.captures(line).and_then(|c| c.name("game_id"));
    match match_option {
        Some(m) => Game {
            id: m.as_str().parse::<u8>().unwrap_or(0),
            rounds: line[m.end()..].split(";").map(|r| parse_round(r)).collect(),
        },
        None => Game::empty(),
    }
}

pub fn solution() {
    let games: Vec<_> = read_input("src/day2/input.txt")
        .split("\n")
        .map(parse_game)
        .collect();

    let valid_games_id_sum: u32 = games
        .iter()
        .filter(|game| !game.is_invalid())
        .fold(0, |sum, game| sum + (game.id as u32));

    println!("Valid game ids sum: {valid_games_id_sum}");

    let power_sum = games.iter().map(|g| g.power()).fold(0, |acc, p| acc + p);

    println!("All game powers sum: {power_sum}");
}
