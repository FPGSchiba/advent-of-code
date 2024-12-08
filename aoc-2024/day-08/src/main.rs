use std::collections::{HashMap, HashSet};
use std::fs;

fn part_one() {
    println!("== PART ONE ==");
    // Read the input file
    let file_path = "input-p1.txt";
    let file_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let lines: Vec<String> = file_str.lines().map(|l| l.to_string()).collect();

    // Parse the map into a 2D grid and collect antennas
    let mut antennas = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push((c, x as i32, y as i32)); // Store frequency and position
            }
        }
    }

    // Set to store unique antinode positions
    let mut antinodes = HashSet::new();

    // Find antinodes for each pair of antennas with the same frequency
    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (freq1, x1, y1) = antennas[i];
            let (freq2, x2, y2) = antennas[j];

            // Only consider antennas with the same frequency
            if freq1 == freq2 {
                // Calculate the vector between the two antennas
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Calculate potential antinode positions
                // Antinode 1: Closer to the first antenna
                let ax1 = x1 - dx;
                let ay1 = y1 - dy;

                // Antinode 2: Farther from the first antenna
                let ax2 = x1 + 2 * dx;
                let ay2 = y1 + 2 * dy;

                // Validate and add antinode 1
                if ax1 >= 0
                    && ay1 >= 0
                    && (ay1 as usize) < lines.len()
                    && (ax1 as usize) < lines[0].len()
                {
                    antinodes.insert((ax1, ay1));
                }

                // Validate and add antinode 2
                if ax2 >= 0
                    && ay2 >= 0
                    && (ay2 as usize) < lines.len()
                    && (ax2 as usize) < lines[0].len()
                {
                    antinodes.insert((ax2, ay2));
                }
            }
        }
    }

    // Output the number of unique antinode locations
    println!("Unique antinode locations: {}", antinodes.len());
}

fn part_two() {
    println!("== PART TWO ==");

    // Read the input file
    let file_path = "input-p2.txt";
    let file_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let lines: Vec<String> = file_str.lines().map(|l| l.to_string()).collect();

    // Determine grid dimensions
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    // Parse the map into a 2D grid and collect antennas by frequency
    let mut antennas_by_frequency: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas_by_frequency
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }

    // Set to store unique antinode positions
    let mut antinodes = HashSet::new();

    // Process each frequency group
    for (_, antennas) in antennas_by_frequency {
        if antennas.len() < 2 {
            // Skip frequencies with only one antenna
            continue;
        }

        // Add all antenna positions as antinodes
        for &(x, y) in &antennas {
            antinodes.insert((x, y));
        }

        // Find antinodes for each pair of antennas
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];

                // Calculate the line between the two antennas
                let dx = x2 - x1;
                let dy = y2 - y1;
                let gcd = gcd(dx.abs(), dy.abs()); // Greatest common divisor to step along the line
                let step_x = dx / gcd;
                let step_y = dy / gcd;

                // Add all positions along the line as antinodes
                for k in 1..gcd {
                    let antinode_x = x1 + k * step_x;
                    let antinode_y = y1 + k * step_y;

                    // Validate and add the antinode if it's within bounds
                    if antinode_x >= 0
                        && antinode_y >= 0
                        && antinode_x < width
                        && antinode_y < height
                    {
                        antinodes.insert((antinode_x, antinode_y));
                    }
                }
            }
        }
    }

    // Output the total number of unique antinode positions
    println!("Unique antinode locations: {}", antinodes.len());
}

// Helper function to calculate the greatest common divisor (GCD)
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    part_one();
    part_two();
}
