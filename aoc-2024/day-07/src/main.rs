use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn perms_iter<'a, T: Copy>(
    input: &'a [T],
    max_len: u32,
) -> impl Iterator<Item = impl Iterator<Item = T> + 'a> {
    (1..=max_len)
        .flat_map(move |len| (0..input.len().pow(len)).zip(std::iter::repeat(len)))
        .map(move |(mut n, j)| {
            (0..j).map(move |_| {
                let s = input[n % input.len()];
                n /= input.len();
                s
            })
        })
}

fn show_eq(values: &Vec<i64>, operations: &Vec<Operation>, result: i64) {
    for (index, value) in values.iter().enumerate() {
        print!("{}", value);
        if index < operations.len() {
            match operations[index] {
                Operation::Add => print!(" + "),
                Operation::Multiply => print!(" * "),
                Operation::Concatenate => print!(" || "),
            }
        }
    }
    println!(" = {}", result);
}

fn evalute_eq(values: &Vec<i64>, result: i64, needed_operations: Vec<Operation>) -> i64 {
    let permutations = perms_iter(&needed_operations, (values.len() - 1) as u32); // Correct length for operators
    let mut eq_final_result = 0;

    for perm in permutations {
        let perm = perm.collect::<Vec<Operation>>();
        if perm.len() != values.len() - 1 {
            continue; // Skip invalid permutations
        }

        let mut eq_result = values[0].to_string();
        for (index, operation) in perm.iter().enumerate() {
            match operation {
                Operation::Add => {
                    eq_result = (eq_result.parse::<i64>().unwrap() + values[index + 1]).to_string();
                }
                Operation::Multiply => {
                    eq_result = (eq_result.parse::<i64>().unwrap() * values[index + 1]).to_string();
                }
                Operation::Concatenate => {
                    eq_result = format!("{}{}", eq_result, values[index + 1]);
                }
            }
        }

        if eq_result.parse::<i64>().unwrap() == result {
            eq_final_result = result; // Store the result if it matches
            break; // Stop checking further permutations once a match is found
        }
    }

    eq_final_result
}

fn part_one() {
    println!("== PART ONE ==");
    let file_path = "input-p1.txt";
    let file_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let lines: Vec<String> = file_str.lines().map(|l| l.to_string()).collect();
    let pb = ProgressBar::new(lines.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) {msg}")
            .expect("Failed to set progress bar style")
            .progress_chars("##-"),
    );
    let mut eq_result_sum = 0;
    for line in lines {
        let str_eq = line.split(':').collect::<Vec<&str>>();
        if str_eq.len() != 2 {
            eprintln!("Invalid line format: {}", line);
            continue; // Skip invalid lines
        }
        let result: i64 = str_eq[0].trim().parse().expect("Failed to parse result");
        let values = str_eq[1]
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse value"))
            .collect::<Vec<i64>>();
        eq_result_sum += evalute_eq(&values, result, vec![Operation::Add, Operation::Multiply]);
        pb.set_message(format!("Result: {}; Values: {:?}", result, values));
        pb.inc(1);
    }
    pb.finish_with_message(format!("Done with result sum of {}", eq_result_sum));
}

fn part_two() {
    println!("== PART TWO ==");
    let file_path = "input-p2.txt";
    let file_str = fs::read_to_string(file_path).unwrap();
    let lines: Vec<String> = file_str.lines().map(|l| l.to_string()).collect();
    let pb = ProgressBar::new(lines.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) {msg}")
            .expect("Failed to set progress bar style")
            .progress_chars("##-"),
    );
    let mut eq_result_sum = 0;
    for line in lines {
        let str_eq = line.split(':').collect::<Vec<&str>>();
        if str_eq.len() != 2 {
            eprintln!("Invalid line format: {}", line);
            continue; // Skip invalid lines
        }
        let result: i64 = str_eq[0].trim().parse().expect("Failed to parse result");
        let values = str_eq[1]
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse value"))
            .collect::<Vec<i64>>();
        eq_result_sum += evalute_eq(
            &values,
            result,
            vec![Operation::Add, Operation::Multiply, Operation::Concatenate],
        );
        pb.set_message(format!("Result: {}; Values: {:?}", result, values));
        pb.inc(1);
    }
    pb.finish_with_message(format!("Done with result sum of {}", eq_result_sum));
}

fn main() {
    part_one();
    part_two();
}
