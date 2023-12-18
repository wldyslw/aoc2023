use crate::util::read_input;
use regex::Regex;
use std::cmp::min;
use std::collections::HashSet;

struct Card {
    id: usize,
    winning: HashSet<usize>,
    actual: HashSet<usize>,
}

impl Card {
    fn empty() -> Card {
        Card {
            id: 0,
            winning: HashSet::new(),
            actual: HashSet::new(),
        }
    }

    fn parse_from_str(str: &str) -> Card {
        let parsed_header = Regex::new(r"Card\s+(?P<card_id>\d+):")
            .unwrap()
            .captures(str)
            .and_then(|c| c.name("card_id"))
            .and_then(|card_id_str| card_id_str.as_str().parse::<usize>().ok());

        let numbers_start_position = str.find(":").unwrap_or(0);

        match parsed_header {
            Some(card_id) => {
                let sorted_numbers = str[numbers_start_position..]
                    .split("|")
                    .map(|substr| {
                        Regex::new(r"(?P<num>\d+)\s*")
                            .unwrap()
                            .captures_iter(substr)
                            .map(|c| c.name("num"))
                            .map(|m| m.and_then(|v| v.as_str().parse::<usize>().ok()))
                            .map(|v| v.unwrap_or_default())
                            .filter(|v| *v > 0)
                            .collect::<HashSet<_>>()
                    })
                    .collect::<Vec<_>>();
                Card {
                    id: card_id,
                    winning: sorted_numbers.get(0).cloned().unwrap(),
                    actual: sorted_numbers.get(1).cloned().unwrap(),
                }
            }
            _ => Card::empty(),
        }
    }

    fn get_matches_count(&self) -> usize {
        self.actual.intersection(&self.winning).count()
    }

    fn get_points(&self) -> usize {
        match self.get_matches_count() {
            0 => 0,
            n => 2_usize.pow(n as u32 - 1),
        }
    }
}

pub fn solution() {
    let cards = read_input("src/day4/input.txt")
        .split("\n")
        .map(|line| Card::parse_from_str(line))
        .collect::<Vec<_>>();
    let cards_number = cards.len();

    let points = cards.iter().fold(0, |acc, c| acc + c.get_points());

    let scratchcards_number = cards
        .iter()
        .map(|c| (c.id - 1, c.get_matches_count()))
        .scan(
            vec![1; cards_number],
            |copies_count, (index, matches_count)| {
                let start = index + 1;
                let end = min(start + matches_count, copies_count.len());

                for i in start..end {
                    copies_count[i] += copies_count[index];
                }
                Some(copies_count[index])
            },
        )
        .fold(0, |acc, s| acc + s);

    println!("Total points: {points}, scratchcards number: {scratchcards_number}")
}
