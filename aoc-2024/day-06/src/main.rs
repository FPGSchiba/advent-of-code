use indicatif::{ProgressBar, ProgressStyle};
use phf::phf_map;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum FieldObject {
    Open,
    Obstacle,
    PlayerUp,
    PlayerDown,
    PlayerLeft,
    PlayerRight,
    Moved,
}

static KEYWORDS: phf::Map<&'static str, FieldObject> = phf_map! {
    "." => FieldObject::Open,
    "#" => FieldObject::Obstacle,
    "^" => FieldObject::PlayerUp,
    ">" => FieldObject::PlayerRight,
    "<" => FieldObject::PlayerLeft,
    "v" => FieldObject::PlayerDown,
};

fn part_one() {
    println!("== PART ONE ==");
    let file_path = "input-p1.txt";
    let file_str = fs::read_to_string(file_path).unwrap();
    let lines: Vec<String> = file_str
        .lines()
        .map(|l| l.to_string())
        .into_iter()
        .collect();

    let mut playing_field: Vec<Vec<&FieldObject>> = Vec::new();
    for line in lines {
        let mut row: Vec<&FieldObject> = Vec::new();
        for char in line.chars() {
            if let Some(object) = KEYWORDS.get(&char.to_string()) {
                row.push(object);
            } else {
                panic!("Unknown character: {}", char);
            }
        }
        playing_field.push(row);
    }

    let mut visited_positions = std::collections::HashSet::new();
    let mut player_pos = None;

    // Find initial player position
    for (i, row) in playing_field.iter().enumerate() {
        for (j, &object) in row.iter().enumerate() {
            match object {
                FieldObject::PlayerUp
                | FieldObject::PlayerDown
                | FieldObject::PlayerLeft
                | FieldObject::PlayerRight => {
                    player_pos = Some((i, j));
                    visited_positions.insert((i, j));
                    break;
                }
                _ => {}
            }
        }
        if player_pos.is_some() {
            break;
        }
    }

    while let Some((i, j)) = player_pos {
        let current_object = playing_field[i][j];

        // Check if we're at the edge of the map
        if (matches!(current_object, FieldObject::PlayerUp) && i == 0)
            || (matches!(current_object, FieldObject::PlayerRight)
                && j == playing_field[0].len() - 1)
            || (matches!(current_object, FieldObject::PlayerDown) && i == playing_field.len() - 1)
            || (matches!(current_object, FieldObject::PlayerLeft) && j == 0)
        {
            break;
        }

        let (next_i, next_j, next_dir) = match current_object {
            FieldObject::PlayerUp => {
                if i > 0 && !matches!(playing_field[i - 1][j], FieldObject::Obstacle) {
                    (i - 1, j, FieldObject::PlayerUp)
                } else {
                    (i, j, FieldObject::PlayerRight)
                }
            }
            FieldObject::PlayerRight => {
                if j < playing_field[0].len() - 1
                    && !matches!(playing_field[i][j + 1], FieldObject::Obstacle)
                {
                    (i, j + 1, FieldObject::PlayerRight)
                } else {
                    (i, j, FieldObject::PlayerDown)
                }
            }
            FieldObject::PlayerDown => {
                if i < playing_field.len() - 1
                    && !matches!(playing_field[i + 1][j], FieldObject::Obstacle)
                {
                    (i + 1, j, FieldObject::PlayerDown)
                } else {
                    (i, j, FieldObject::PlayerLeft)
                }
            }
            FieldObject::PlayerLeft => {
                if j > 0 && !matches!(playing_field[i][j - 1], FieldObject::Obstacle) {
                    (i, j - 1, FieldObject::PlayerLeft)
                } else {
                    (i, j, FieldObject::PlayerUp)
                }
            }
            _ => break,
        };

        // Update the playing field
        if (next_i, next_j) != (i, j) {
            playing_field[i][j] = &FieldObject::Moved;
            playing_field[next_i][next_j] = match next_dir {
                FieldObject::PlayerUp => &FieldObject::PlayerUp,
                FieldObject::PlayerRight => &FieldObject::PlayerRight,
                FieldObject::PlayerDown => &FieldObject::PlayerDown,
                FieldObject::PlayerLeft => &FieldObject::PlayerLeft,
                _ => unreachable!(),
            };
            visited_positions.insert((next_i, next_j));
            player_pos = Some((next_i, next_j));
        } else {
            // Just changing direction
            playing_field[i][j] = match next_dir {
                FieldObject::PlayerUp => &FieldObject::PlayerUp,
                FieldObject::PlayerRight => &FieldObject::PlayerRight,
                FieldObject::PlayerDown => &FieldObject::PlayerDown,
                FieldObject::PlayerLeft => &FieldObject::PlayerLeft,
                _ => unreachable!(),
            };
        }
    }

    println!("Number of visited positions: {}", visited_positions.len());
}

fn simulate_path(
    playing_field: &Vec<Vec<&FieldObject>>,
    obstacle_pos: Option<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut field_copy = playing_field.clone();
    let mut visited = HashSet::new();
    let mut position_direction_history = HashSet::new();

    // If we have an obstacle position, add it to the field
    if let Some((ox, oy)) = obstacle_pos {
        field_copy[oy][ox] = &FieldObject::Obstacle;
    }

    // Find start position and direction
    let mut current_pos = None;
    let mut current_dir = None;
    for (y, row) in field_copy.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                FieldObject::PlayerUp => {
                    current_pos = Some((x, y));
                    current_dir = Some(FieldObject::PlayerUp);
                }
                FieldObject::PlayerDown => {
                    current_pos = Some((x, y));
                    current_dir = Some(FieldObject::PlayerDown);
                }
                FieldObject::PlayerLeft => {
                    current_pos = Some((x, y));
                    current_dir = Some(FieldObject::PlayerLeft);
                }
                FieldObject::PlayerRight => {
                    current_pos = Some((x, y));
                    current_dir = Some(FieldObject::PlayerRight);
                }
                _ => {}
            }
        }
    }

    while let (Some((x, y)), Some(dir)) = (current_pos, current_dir) {
        // Add current position to visited set
        visited.insert((x, y));

        // Check if we've been in this position with this direction before
        let state = (x, y, std::mem::discriminant(&dir));
        if !position_direction_history.insert(state) {
            // We found a loop!
            return visited;
        }

        // Calculate next position based on direction
        let (next_x, next_y) = match dir {
            FieldObject::PlayerUp => (x, y.wrapping_sub(1)),
            FieldObject::PlayerDown => (x, y + 1),
            FieldObject::PlayerLeft => (x.wrapping_sub(1), y),
            FieldObject::PlayerRight => (x + 1, y),
            _ => unreachable!(),
        };

        // Check if we're leaving the map
        if next_y >= field_copy.len() || next_x >= field_copy[0].len() {
            visited.clear(); // Clear visited since this is not a loop
            break;
        }

        // Check if there's an obstacle ahead
        if matches!(field_copy[next_y][next_x], FieldObject::Obstacle) {
            // Turn right
            current_dir = Some(match dir {
                FieldObject::PlayerUp => FieldObject::PlayerRight,
                FieldObject::PlayerRight => FieldObject::PlayerDown,
                FieldObject::PlayerDown => FieldObject::PlayerLeft,
                FieldObject::PlayerLeft => FieldObject::PlayerUp,
                _ => unreachable!(),
            });
            current_pos = Some((x, y));
        } else {
            current_pos = Some((next_x, next_y));
        }
    }

    visited
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

    let mut playing_field: Vec<Vec<&FieldObject>> = Vec::new();
    for line in lines {
        let mut row: Vec<&FieldObject> = Vec::new();
        for char in line.chars() {
            if let Some(object) = KEYWORDS.get(&char.to_string()) {
                row.push(object);
            } else {
                panic!("Unknown character: {}", char);
            }
        }
        playing_field.push(row);
    }

    // Find start position and count total possible positions
    let mut start_pos = None;
    let mut total_positions = 0;
    for (y, row) in playing_field.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                FieldObject::PlayerUp
                | FieldObject::PlayerDown
                | FieldObject::PlayerLeft
                | FieldObject::PlayerRight => {
                    start_pos = Some((x, y));
                }
                FieldObject::Open | FieldObject::Moved => {
                    total_positions += 1;
                }
                _ => {}
            }
        }
    }

    // Setup progress bar
    let pb = ProgressBar::new(total_positions);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    // Try each possible position for the new obstacle
    let mut valid_positions = 0;
    for y in 0..playing_field.len() {
        for x in 0..playing_field[0].len() {
            // Skip if position is already occupied or is the start position
            match playing_field[y][x] {
                FieldObject::Obstacle
                | FieldObject::PlayerUp
                | FieldObject::PlayerDown
                | FieldObject::PlayerLeft
                | FieldObject::PlayerRight => {
                    continue;
                }
                _ => {}
            }

            if Some((x, y)) == start_pos {
                continue;
            }

            // Simulate path with new obstacle
            let visited = simulate_path(&playing_field, Some((x, y)));

            // If the path forms a loop (doesn't reach the edge), it's a valid position
            if !visited.is_empty() && visited.len() < playing_field.len() * playing_field[0].len() {
                valid_positions += 1;
                pb.set_message(format!("Found {} valid positions", valid_positions));
            }

            pb.inc(1);
        }
    }

    pb.finish_with_message(format!(
        "Complete! Found {} valid positions",
        valid_positions
    ));
}

fn main() {
    part_one();
    part_two();
}
