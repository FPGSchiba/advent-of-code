use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::{fs, num};

// Block Size is 4x4
fn evaluate_block(block: Vec<&String>) -> usize {
    let mut num_xmas_in_block = 0;
    // Diagonally evaluate the block (top-left to bottom-right)
    let mut diagonal_line = String::new();
    for i in 0..block.len() {
        diagonal_line.push(block[i].chars().nth(i).unwrap());
    }
    if diagonal_line == "XMAS" || diagonal_line == "SAMX" {
        num_xmas_in_block += 1;
    }

    // Diagonally evaluate the block (top-right to bottom-left)
    let mut diagonal_line = String::new();
    for i in 0..block.len() {
        diagonal_line.push(block[i].chars().nth(block.len() - 1 - i).unwrap());
    }
    if diagonal_line == "XMAS" || diagonal_line == "SAMX" {
        num_xmas_in_block += 1;
    }

    num_xmas_in_block
}

fn evaluate_x_mas(block: Vec<&String>) -> usize {
    let mut num_xmas_in_block = 0;
    // Diagonally evaluate the block (top-left to bottom-right)
    let mut diagonal_line = String::new();
    for i in 0..block.len() {
        diagonal_line.push(block[i].chars().nth(i).unwrap());
    }
    if diagonal_line == "MAS" || diagonal_line == "SAM" {
        num_xmas_in_block += 1;
    }

    // Diagonally evaluate the block (top-right to bottom-left)
    let mut diagonal_line = String::new();
    for i in 0..block.len() {
        diagonal_line.push(block[i].chars().nth(block.len() - 1 - i).unwrap());
    }
    if diagonal_line == "MAS" || diagonal_line == "SAM" {
        num_xmas_in_block += 1;
    }

    if num_xmas_in_block == 2 {
        return 1;
    }

    0
}

fn part_one() {
    println!("== PART ONE ==");
    let file_path = "input-p1.txt";
    let file_str = fs::read_to_string(file_path).unwrap();
    let lines: Vec<String> = file_str
        .lines()
        .map(|l| l.to_string())
        .into_iter()
        .collect();
    let mut xmas_sum = 0;
    for i in 0..(lines.len() - 3) {
        for j in 0..(lines[0].len() - 3) {
            let line1 = lines[i][j..j + 4].to_string();
            let line2 = lines[i + 1][j..j + 4].to_string();
            let line3 = lines[i + 2][j..j + 4].to_string();
            let line4 = lines[i + 3][j..j + 4].to_string();
            let block = vec![&line1, &line2, &line3, &line4];
            xmas_sum += evaluate_block(block);
        }
    }

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(XMAS|SAMX)").unwrap();
    }
    // iterate over all matches
    let results: Vec<Match> = RE.find_iter(&file_str).collect();
    xmas_sum += results.len();

    let mut new_lines: Vec<String> = vec![String::new(); lines.len()];
    for line in lines {
        for i in 0..line.len() {
            new_lines[i].push(char::from(line.chars().nth(i).unwrap()));
        }
    }

    let new_string = new_lines.join("");
    // iterate over all matches
    let results: Vec<Match> = RE.find_iter(&new_string).collect();
    xmas_sum += results.len();

    println!("Number of XMAS in all Blocks: {}", xmas_sum);
}

fn part_two() {
    println!("== PART TWO ==");
    let file_path = "input-p2.txt";
    let file_str = fs::read_to_string(file_path).unwrap();
    let lines: Vec<String> = file_str
        .lines()
        .map(|l| l.to_string())
        .into_iter()
        .collect();
    let mut xmas_sum = 0;
    for i in 0..(lines.len() - 2) {
        for j in 0..(lines[0].len() - 2) {
            let line1 = lines[i][j..j + 3].to_string();
            let line2 = lines[i + 1][j..j + 3].to_string();
            let line3 = lines[i + 2][j..j + 3].to_string();
            let block = vec![&line1, &line2, &line3];
            xmas_sum += evaluate_x_mas(block);
        }
    }
    println!("Number of X-MAS in all Blocks: {}", xmas_sum);
}

fn main() {
    part_one();
    part_two();
}
