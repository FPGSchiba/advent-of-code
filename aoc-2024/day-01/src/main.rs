use std::fs;

fn part_one() {
    let file_path = "input-p1.txt";
    let mut team_a: Vec<i32> = Vec::new();
    let mut team_b: Vec<i32> = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let line_string = line.to_string();
        let values = line_string.split("   ").collect::<Vec<&str>>();
        team_a.push(values[0].parse::<i32>().unwrap());
        team_b.push(values[1].parse::<i32>().unwrap());
    }
    team_a.sort();
    team_b.sort();
    let complete_distance = team_a
        .iter()
        .zip(team_b.iter())
        .map(|(a, b)| (b - a).abs())
        .sum::<i32>();
    println!("Complete distance: {}", complete_distance);
}

fn part_two() {
    let file_path = "input-p2.txt";
    let mut team_a: Vec<i32> = Vec::new();
    let mut team_b: Vec<i32> = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let line_string = line.to_string();
        let values = line_string.split("   ").collect::<Vec<&str>>();
        team_a.push(values[0].parse::<i32>().unwrap());
        team_b.push(values[1].parse::<i32>().unwrap());
    }
    let mut total_distance = 0;
    for value in team_a.iter() {
        total_distance += team_b.iter().filter(|&n| n == value).count() as i32 * value;
    }
    println!("Total distance: {}", total_distance);
}

fn main() {
    part_one();
    part_two();
}
