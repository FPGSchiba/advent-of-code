use std::fs;

#[derive(Debug)]
enum FileBlock {
    Empty,
    File(i64),
}

fn get_index_of_first_empty(file_blocks: &Vec<FileBlock>) -> i32 {
    for (index, block) in file_blocks.iter().enumerate() {
        match block {
            FileBlock::Empty => return index as i32,
            _ => {}
        }
    }
    return -1;
}

fn part_one() {
    println!("== PART ONE ==");
    let file_path = "input-p1.txt";
    let file_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let mut file_blocks: Vec<FileBlock> = Vec::new();
    let mut is_file = true;
    let mut index = 0;
    for char in file_str.chars() {
        let current_number = char.to_string().parse::<i32>().unwrap();
        if is_file {
            for _ in 0..current_number {
                file_blocks.push(FileBlock::File(index as i64));
            }
            is_file = false;
        } else {
            for _ in 0..current_number {
                file_blocks.push(FileBlock::Empty);
            }
            is_file = true;
            index += 1;
        }
    }
    for rev_index in (0..file_blocks.len()).rev() {
        match file_blocks[rev_index] {
            FileBlock::Empty => {}
            FileBlock::File(_) => {
                let empty_index = get_index_of_first_empty(&file_blocks);
                if empty_index > rev_index as i32 {
                    break;
                }
                if empty_index != -1 {
                    file_blocks.swap(rev_index, empty_index as usize);
                }
            }
        }
    }
    let mut hash_sum: i64 = 0;
    for (index, block) in file_blocks.iter().enumerate() {
        match block {
            FileBlock::Empty => {}
            FileBlock::File(id) => hash_sum += (index as i64 * id) as i64,
        }
    }
    println!("Hash sum: {}", hash_sum);
}

fn part_two() {
    println!("== PART TWO ==");
    let file_path = "input-p2.txt";
    let file_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let mut file_blocks: Vec<FileBlock> = Vec::new();
    let mut is_file = true;
    let mut index = 0;

    // Parse the input file into file_blocks
    for char in file_str.chars() {
        let current_number = char.to_string().parse::<i32>().unwrap();
        if is_file {
            for _ in 0..current_number {
                file_blocks.push(FileBlock::File(index as i64));
            }
            is_file = false;
        } else {
            for _ in 0..current_number {
                file_blocks.push(FileBlock::Empty);
            }
            is_file = true;
            index += 1;
        }
    }

    // Get the list of unique file IDs in descending order
    let mut file_ids: Vec<i64> = file_blocks
        .iter()
        .filter_map(|block| match block {
            FileBlock::File(id) => Some(*id),
            _ => None,
        })
        .collect();
    file_ids.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    // Attempt to move each file to the leftmost span of free space
    for file_id in file_ids {
        // Find the current position and size of the file
        let mut file_start = None;
        let mut file_size = 0;

        for (i, block) in file_blocks.iter().enumerate() {
            match block {
                FileBlock::File(id) if *id == file_id => {
                    if file_start.is_none() {
                        file_start = Some(i);
                    }
                    file_size += 1;
                }
                _ => {
                    if file_start.is_some() {
                        break;
                    }
                }
            }
        }

        let file_start = file_start.unwrap();

        // Find the leftmost span of free space that can fit the file
        let mut free_start = None;
        let mut free_size = 0;

        for (i, block) in file_blocks.iter().enumerate() {
            match block {
                FileBlock::Empty => {
                    if free_start.is_none() {
                        free_start = Some(i);
                    }
                    free_size += 1;

                    if free_size >= file_size {
                        break;
                    }
                }
                _ => {
                    free_start = None;
                    free_size = 0;
                }
            }
        }

        // If a suitable span of free space is found, move the file
        if let Some(free_start) = free_start {
            if free_size >= file_size && free_start < file_start {
                // Clear the original file blocks
                for i in file_start..file_start + file_size {
                    file_blocks[i] = FileBlock::Empty;
                }

                // Move the file to the new location
                for i in free_start..free_start + file_size {
                    file_blocks[i] = FileBlock::File(file_id);
                }
            }
        }
    }

    // Calculate the checksum
    let mut hash_sum: i64 = 0;
    for (index, block) in file_blocks.iter().enumerate() {
        match block {
            FileBlock::Empty => {}
            FileBlock::File(id) => hash_sum += (index as i64 * id) as i64,
        }
    }
    println!("Hash sum: {}", hash_sum);
}

fn main() {
    part_one();
    part_two();
}
