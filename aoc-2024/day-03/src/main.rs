use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::fs;

fn part_one() {
    println!("== PART ONE ==");
    let file_path = "input-p1.txt";
    let file_str = fs::read_to_string(file_path).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    }
    // iterate over all matches
    let results: Vec<Match> = RE.find_iter(&file_str).collect();
    let mut mul_sum = 0;
    for result in results {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d{1,3}").unwrap();
        }
        // iterate over all matches
        let results: Vec<i32> = RE
            .find_iter(result.as_str())
            .filter_map(|instruction| Some(instruction.as_str().parse().unwrap()))
            .collect();
        mul_sum += results[0] * results[1];
    }
    println!("Sum of all mul Instructions: {}", mul_sum);
}

fn part_two() {
    println!("== PART TWO ==");
    let file_path = "input-p2.txt";
    let file_str = fs::read_to_string(file_path).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    }
    // iterate over all matches
    let results: Vec<Match> = RE.find_iter(&file_str).collect();
    let mut mul_sum = 0;
    let mut enabled = true;
    for result in results {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d{1,3}").unwrap();
        }
        let str_result = result.as_str();
        println!("Result: {}", str_result);
        match str_result {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => {
                if !enabled {
                    continue;
                }
                let results: Vec<i32> = RE
                    .find_iter(result.as_str())
                    .filter_map(|instruction| Some(instruction.as_str().parse().unwrap()))
                    .collect();
                mul_sum += results[0] * results[1];
            }
        }
    }
    println!("Sum of all mul Instructions: {}", mul_sum);
}

fn main() {
    part_one();
    part_two();
}
