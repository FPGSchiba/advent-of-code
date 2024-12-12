use std::collections::{HashSet, VecDeque};
use std::fs;

struct Region {
    area: u64,
    perimeter: u64,
    num_sides: u64,
    plant: String,
}

fn get_regions(map: Vec<Vec<char>>) -> Vec<Region> {
    let mut regions = Vec::new();
    let rows = map.len();
    let cols = map[0].len();
    let mut visited = HashSet::new();

    // Helper function to check if a position is valid
    let is_valid =
        |r: i32, c: i32| -> bool { r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 };

    // Directions for adjacent cells (right, down, left, up)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for i in 0..rows {
        for j in 0..cols {
            if visited.contains(&(i, j)) {
                continue;
            }

            let current_plant = map[i][j];
            let mut queue = VecDeque::new();
            queue.push_back((i, j));
            visited.insert((i, j));

            let mut area = 0;
            let mut perimeter = 0;
            let mut region_cells = HashSet::new();
            let mut horizontal_edges = HashSet::new();
            let mut vertical_edges = HashSet::new();

            // Flood fill to find connected cells of the same plant
            while let Some((r, c)) = queue.pop_front() {
                area += 1;
                region_cells.insert((r, c));

                for (dir, (dr, dc)) in directions.iter().enumerate() {
                    let new_r = r as i32 + dr;
                    let new_c = c as i32 + dc;

                    if !is_valid(new_r, new_c) {
                        perimeter += 1;
                        // Add edge to appropriate set
                        match dir {
                            0 | 2 => horizontal_edges.insert((r, c.min(c + 1))),
                            1 | 3 => vertical_edges.insert((r.min(r + 1), c)),
                            _ => unreachable!(),
                        };
                        continue;
                    }

                    let new_r = new_r as usize;
                    let new_c = new_c as usize;

                    if map[new_r][new_c] != current_plant {
                        perimeter += 1;
                        // Add edge to appropriate set
                        match dir {
                            0 | 2 => horizontal_edges.insert((r, c.min(new_c))),
                            1 | 3 => vertical_edges.insert((r.min(new_r), c)),
                            _ => unreachable!(),
                        };
                    } else if !visited.contains(&(new_r, new_c)) {
                        queue.push_back((new_r, new_c));
                        visited.insert((new_r, new_c));
                    }
                }
            }

            // Total number of sides is the sum of unique horizontal and vertical edges
            let num_sides = (horizontal_edges.len() + vertical_edges.len()) as u64;

            regions.push(Region {
                area: area,
                perimeter: perimeter,
                num_sides: num_sides,
                plant: current_plant.to_string(),
            });
        }
    }

    regions
}

fn part_one() {
    println!("== PART ONE ==");
    let file_path = "input-p1.txt";
    let file_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let map = file_str
        .lines()
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let regions = get_regions(map);
    let mut total_cost = 0;
    for region in regions {
        total_cost += region.area * region.perimeter;
    }
    println!("Total cost: {}", total_cost);
}

fn part_two() {
    println!("== PART TWO ==");
    let file_path = "input-p2.txt";
    let file_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let map = file_str
        .lines()
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let regions = get_regions(map);
    let mut total_cost = 0;
    for region in regions {
        total_cost += region.area * region.num_sides;
    }
    println!("Total cost: {}", total_cost);
}

fn main() {
    part_one();
    part_two();
}
