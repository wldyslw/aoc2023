use std::env;

mod day1;

fn pick_day(day: u8) {
    match day {
        1 => day1::solution(),
        2..=24 => println!("Yet to be implemented"),
        _ => (),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1);
    match day {
        Some(day) => {
            let num = day.parse::<u8>().unwrap_or(0);
            if num == 0 || num > 24 {
                println!("Invalid day number");
            } else {
                pick_day(num);
            }
        }
        None => println!("Please provide a day number"),
    }
}