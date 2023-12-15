use crate::util::read_input;
use regex::{Match, Regex};

fn to_digit(input: &str) -> Option<u32> {
    match input {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        "0" | "zero" => Some(0),
        _ => None,
    }
}

pub fn solution() {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine|zero)").unwrap();
    let mut sum = 0;

    for line in read_input("src/day1/input.txt").split('\n') {
        let all_matches: Vec<Match> = line
            .char_indices()
            .filter_map(|(i, _c)| re.find_at(line, i))
            .collect();

        let first = all_matches.first().and_then(|m| to_digit(m.as_str()));
        let last = all_matches.last().and_then(|m| to_digit(m.as_str()));

        sum += match (first, last) {
            (Some(f), Some(l)) => f * 10 + l,
            _ => 0,
        };
    }

    println!("SUM: {sum}");
}
