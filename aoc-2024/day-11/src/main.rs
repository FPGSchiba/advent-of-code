use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

fn calculate_next_blink(file_path: &str, num_blinks: u8) {
    let stones = fs::read_to_string(file_path)
        .expect("Failed to read input file")
        .split(" ")
        .map(|x| x.parse::<u64>().expect("Failed to parse input"))
        .collect::<Vec<u64>>();
    let mut stones = stones;
    let pb = ProgressBar::new(num_blinks as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) {msg}")
            .expect("Failed to set progress bar style")
            .progress_chars("##-"),
    );
    for _ in 1..num_blinks + 1 {
        let mut new_stones = vec![];
        for j in 0..stones.len() {
            if stones[j] == 0 {
                new_stones.push(1);
            } else if (0..).take_while(|i| 10u64.pow(*i) <= stones[j]).count() % 2 == 0 {
                let num_digits = (0..).take_while(|i| 10u64.pow(*i) <= stones[j]).count();
                new_stones.push(
                    stones[j]
                        .to_string()
                        .chars()
                        .map(|d| d.to_digit(10).unwrap())
                        .collect::<Vec<_>>()[0..num_digits / 2]
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                        .parse::<u64>()
                        .unwrap(),
                );
                new_stones.push(
                    stones[j]
                        .to_string()
                        .chars()
                        .map(|d| d.to_digit(10).unwrap())
                        .collect::<Vec<_>>()[num_digits / 2..num_digits]
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                        .parse::<u64>()
                        .unwrap(),
                );
            } else {
                new_stones.push(stones[j] * 2024);
            }
        }
        stones = new_stones;
        pb.set_message(format!("Number of Stones: {}", stones.len()));
        pb.inc(1);
    }
    pb.finish_with_message(format!("Number of final Stones {}", stones.len()));
}

fn part_one() {
    println!("== PART ONE ==");
    let file_path = "input-p1.txt";
    calculate_next_blink(file_path, 25);
}

fn part_two() {
    println!("== PART TWO ==");
    let file_path = "input-p2.txt";
    calculate_next_blink(file_path, 75);
}

fn main() {
    part_one();
    part_two();
}
