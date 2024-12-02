use std::fs;

fn is_report_save(report: Vec<i32>) -> bool {
    let mut save = 1;
    let mut is_increasing = true;
    for (index, value) in report.iter().enumerate() {
        if index == report.len() - 1 {
            break;
        }
        let dist_forward = value - report[index + 1];
        if index == 0 {
            is_increasing = dist_forward > 0;
        }
        if is_increasing && dist_forward < 0 {
            save = 0;
            break;
        } else if !is_increasing && dist_forward > 0 {
            save = 0;
            break;
        }
        if dist_forward.abs() > 3 || dist_forward.abs() == 0 {
            save = 0;
            break;
        }
    }
    return save == 1;
}

fn part_one() {
    println!("== PART ONE ==");
    let file_path = "input-p1.txt";
    let mut num_save_reports = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let line_string = line.to_string();
        let values = line_string.split(" ").collect::<Vec<&str>>();
        let report: Vec<i32> = values.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        num_save_reports += is_report_save(report) as i32;
    }
    println!("(Without Dampener) Save Reports: {}", num_save_reports);
}

fn is_report_save_v2(report: Vec<i32>) -> bool {
    if is_report_save(report.clone()) {
        return true;
    }
    for (index, _) in report.iter().enumerate() {
        let mut report_clone = report.clone();
        report_clone.remove(index);
        if is_report_save(report_clone.clone()) {
            return true;
        }
    }
    return false;
}

fn part_two() {
    println!("== PART TWO ==");
    let file_path = "input-p2.txt";
    let mut num_save_reports = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let line_string = line.to_string();
        let values = line_string.split(" ").collect::<Vec<&str>>();
        let report: Vec<i32> = values.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        if is_report_save_v2(report.clone()) {
            num_save_reports += 1;
        }
    }
    println!("(With Dampener) Save reports: {}", num_save_reports);
}

fn main() {
    part_one();
    part_two();
}
