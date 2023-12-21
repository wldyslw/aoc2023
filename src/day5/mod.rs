use crate::util::read_input;
use regex::Regex;
use std::cmp::min;
use std::fmt::Debug;

type Seed = usize;

#[derive(Debug)]
struct MapRange {
    destination: usize,
    source: usize,
    len: usize,
}

impl MapRange {
    fn convert(&self, seed: Seed) -> Option<Seed> {
        let min = self.source;
        let max = self.source + self.len;
        if seed <= max && seed >= min {
            Some(self.destination + (seed - self.source))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

fn parse_input(input: &str) -> (Vec<Seed>, Vec<Map>) {
    let chunks = input.split("\n\n");

    let seeds = chunks
        .clone()
        .take(1)
        .collect::<Vec<_>>()
        .first()
        .and_then(|f| {
            Some(
                f.split(" ")
                    .skip(1)
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap_or_default();

    let maps = chunks
        .skip(1)
        .map(|i| {
            Regex::new(r"(?P<destination>\d+) (?P<source>\d+) (?P<len>\d+)")
                .unwrap()
                .captures_iter(i)
                .map(|c| {
                    (
                        c.name("destination")
                            .and_then(|m| m.as_str().parse::<usize>().ok())
                            .unwrap_or(0),
                        c.name("source")
                            .and_then(|m| m.as_str().parse::<usize>().ok())
                            .unwrap_or(0),
                        c.name("len")
                            .and_then(|m| m.as_str().parse::<usize>().ok())
                            .unwrap_or(0),
                    )
                })
                .map(|(destination, source, len)| MapRange {
                    destination,
                    source,
                    len,
                })
                .collect::<Vec<_>>()
        })
        .map(|ranges| Map { ranges })
        .collect::<Vec<_>>();

    (seeds, maps)
}

fn get_min_location(seeds: &Vec<Seed>, maps: &Vec<Map>) -> Seed {
    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |acc, m| {
                m.ranges.iter().find_map(|r| r.convert(acc)).unwrap_or(acc)
            })
        })
        .min()
        .unwrap_or(0)
}

pub fn solution() {
    let (seeds, maps) = parse_input(&read_input("src/day5/input.txt"));

    let min1 = get_min_location(&seeds, &maps);

    let min2 = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .inspect(|(from, len)| println!("{:.<12}{:.>12}", from, **from + **len))
        .map(|(from, len)| (*from..=(*from + *len)))
        .fold(usize::MAX, |acc, range| {
            min(acc, get_min_location(&range.collect::<Vec<_>>(), &maps))
        });

    println!("\nMIN LOCATION 1: {:?}, MIN LOCATION 2: {:?}", min1, min2);
}
