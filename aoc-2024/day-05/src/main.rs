use std::fs;

fn are_pages_sorted(pages: &Vec<i32>, rule_map: &Vec<(i32, i32)>) -> bool {
    let mut sorted = true;
    for (rule_x, rule_y) in rule_map.iter() {
        if let (Some(x_index), Some(y_index)) = (
            pages.iter().position(|&r| r == *rule_x),
            pages.iter().position(|&r| r == *rule_y),
        ) {
            if x_index > y_index {
                sorted = false;
            }
        }
    }
    sorted
}

fn sort_pages(pages: &Vec<i32>, rule_map: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut sorted_pages = pages.clone();
    let mut changed = true;

    // Keep swapping until no more changes are needed
    while changed {
        changed = false;
        for (rule_x, rule_y) in rule_map.iter() {
            if let (Some(x_index), Some(y_index)) = (
                sorted_pages.iter().position(|&r| r == *rule_x),
                sorted_pages.iter().position(|&r| r == *rule_y),
            ) {
                // If we find a pair that violates the rule (x should be before y)
                if x_index > y_index {
                    // Swap the elements
                    sorted_pages.swap(x_index, y_index);
                    changed = true;
                }
            }
        }
    }

    sorted_pages
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
    let split_index = lines.iter().position(|l| l == "").unwrap();
    let rules = &lines[..split_index];
    let messages = &lines[split_index + 1..];

    // Parse rules
    let mut rule_map: Vec<(i32, i32)> = vec![];
    for rule in rules {
        let rule_parts: Vec<&str> = rule.split("|").collect();
        let rule_x = rule_parts[0].parse::<i32>().unwrap();
        let rule_y = rule_parts[1].trim().parse::<i32>().unwrap();
        rule_map.push((rule_x, rule_y));
    }

    let mut mid_page_sum = 0;

    // Process each message
    for message in messages {
        let page_numbers: Vec<i32> = message
            .split(",")
            .map(|p| p.parse::<i32>().unwrap())
            .collect();

        // Only add middle page if the order is correct
        if are_pages_sorted(&page_numbers, &rule_map) {
            let mid_index = (page_numbers.len() - 1) / 2;
            let mid_page_number = page_numbers[mid_index];
            mid_page_sum += mid_page_number;
        }
    }

    println!("Mid Page Sum: {}", mid_page_sum);
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
    let split_index = lines.iter().position(|l| l == "").unwrap();
    let rules = &lines[..split_index];
    let messages = &lines[split_index + 1..];

    // Parse rules
    let mut rule_map: Vec<(i32, i32)> = vec![];
    for rule in rules {
        let rule_parts: Vec<&str> = rule.split("|").collect();
        let rule_x = rule_parts[0].parse::<i32>().unwrap();
        let rule_y = rule_parts[1].trim().parse::<i32>().unwrap();
        rule_map.push((rule_x, rule_y));
    }

    let mut mid_page_sum = 0;

    // Process each message
    for message in messages {
        let page_numbers: Vec<i32> = message
            .split(",")
            .map(|p| p.parse::<i32>().unwrap())
            .collect();

        // Only process incorrectly ordered updates
        if !are_pages_sorted(&page_numbers, &rule_map) {
            let sorted_pages = sort_pages(&page_numbers, &rule_map);
            if are_pages_sorted(&sorted_pages, &rule_map) {
                let mid_index = (sorted_pages.len() - 1) / 2;
                let mid_page_number = sorted_pages[mid_index];
                mid_page_sum += mid_page_number;
            } else {
                println!("Could not sort correctly: {:?}", page_numbers);
            }
        }
    }

    println!(
        "Sum of middle pages from corrected updates: {}",
        mid_page_sum
    );
}

fn main() {
    part_one();
    part_two();
}
