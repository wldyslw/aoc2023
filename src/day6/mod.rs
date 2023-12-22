use crate::util::read_input;

fn parse_line(line: &str) -> impl Iterator<Item = u64> + '_ {
    line.split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split_once("\n")
        .and_then(|(t, d)| Some(parse_line(t).zip(parse_line(d)).collect::<Vec<_>>()))
        .unwrap_or(Vec::new())
}

fn parse_line_concat(line: &str) -> u64 {
    line.split_once(":")
        .and_then(|(_, nums)| nums.replace(" ", "").parse::<u64>().ok())
        .unwrap_or_default()
}

fn parse_input_concat(input: &str) -> (u64, u64) {
    input
        .split_once("\n")
        .and_then(|(t, d)| Some((parse_line_concat(t), parse_line_concat(d))))
        .unwrap_or_default()
}

/**
 * Solving x * (t_max - x) - d_min = 0, where:
 * x - start time
 * t_max - race max time
 * d_min - previous record (or minimum distance to travel)
 */
fn find_better_times_count(t_max: u64, d_min: u64) -> u64 {
    let discriminant_root = ((t_max * t_max - 4 * d_min) as f64).sqrt();
    let min = (((t_max as f64) - discriminant_root) / 2_f64).floor() as u64 + 1;
    let max = (((t_max as f64) + discriminant_root) / 2_f64).ceil() as u64;
    max - min
}

pub fn solution() {
    let input = read_input("src/day6/input.txt");
    let pairs = parse_input(&input);
    let pair_concat = parse_input_concat(&input);

    let part_1 = pairs
        .iter()
        .fold(1, |acc, (t, d)| acc * find_better_times_count(*t, *d));
    println!("{part_1}");

    let part_2 = find_better_times_count(pair_concat.0, pair_concat.1);
    println!("{part_2}");
}
