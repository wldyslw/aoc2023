use regex::Regex;

use crate::util::read_input;

struct Number {
    value: u32,
    col_start: usize,
    col_end: usize,
    row: usize,
}

pub fn solution() {
    let re = Regex::new(r"\d+").unwrap();
    let input = read_input("src/day3/input.txt");
    let numbers: Vec<Number> = input
        .split("\n")
        .enumerate()
        .flat_map(|(row, line)| {
            re.find_iter(line).map(move |m| Number {
                value: m.as_str().parse::<u32>().unwrap(),
                col_start: m.start(),
                col_end: m.end() - 1,
                row,
            })
        })
        .collect();

    let mut sum = 0_u32;
    let mut ratio = 0_u32;

    for (row, line) in input.split("\n").enumerate() {
        for (col, character) in line.char_indices() {
            let (s, r) = match character {
                '0'..='9' | '.' => (0_u32, 0_u32),
                other => {
                    let adjacent_nums: Vec<u32> = numbers
                        .iter()
                        .filter_map(|n| {
                            let is_row_adjacent = [-1, 0, 1]
                                .iter()
                                .any(|offset| row == n.row.saturating_add_signed(*offset));
                            let is_col_adjacent =
                                col >= n.col_start.saturating_sub(1) && col <= n.col_end + 1;
                            if is_row_adjacent && is_col_adjacent {
                                Some(n.value)
                            } else {
                                None
                            }
                        })
                        .collect();

                    (
                        adjacent_nums.iter().sum(),
                        if other == '*' && adjacent_nums.len() == 2 {
                            adjacent_nums.iter().product()
                        } else {
                            0
                        },
                    )
                }
            };
            sum += s;
            ratio += r;
        }
    }

    println!("ADJACENT NUMBERS SUM: {sum}, GEAR RATIO SUM: {ratio}");
}
