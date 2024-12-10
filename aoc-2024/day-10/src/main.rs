use std::collections::HashMap;
use std::fs;

fn part_two() {
    println!("== PART TWO ==");
    let file_path = "input-p2.txt";
    let file_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let lines = file_str.lines().collect::<Vec<&str>>();

    // Parse the input into a 2D vector of integers
    let map: Vec<Vec<u8>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let rows = map.len();
    let cols = map[0].len();

    // Find all trailheads (positions with height 0)
    let mut trailheads = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                trailheads.push((r, c));
            }
        }
    }

    // Memoization map to store the number of distinct paths from each position
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    // Function to perform DFS and count distinct paths from a given position
    fn dfs(
        map: &Vec<Vec<u8>>,
        r: usize,
        c: usize,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        // If this position is already memoized, return the stored result
        if let Some(&count) = memo.get(&(r, c)) {
            return count;
        }

        let rows = map.len();
        let cols = map[0].len();

        // If this position is a 9, it is a valid endpoint for a trail
        if map[r][c] == 9 {
            return 1;
        }

        let mut total_paths = 0;

        // Directions for moving up, down, left, right
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (dr, dc) in directions.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let nr = nr as usize;
                let nc = nc as usize;

                // Check if the next position is a valid step (height increases by 1)
                if map[nr][nc] == map[r][c] + 1 {
                    total_paths += dfs(map, nr, nc, memo);
                }
            }
        }

        // Store the result in the memoization map
        memo.insert((r, c), total_paths);
        total_paths
    }

    // Calculate the rating for each trailhead
    let mut total_rating = 0;
    for trailhead in trailheads {
        total_rating += dfs(&map, trailhead.0, trailhead.1, &mut memo);
    }

    println!("Total rating of all trailheads: {}", total_rating);
}

fn part_one() {
    println!("== PART ONE ==");
    let file_path = "input-p1.txt";
    let file_str = fs::read_to_string(file_path).expect("Failed to read input file");
    let lines = file_str.lines().collect::<Vec<&str>>();

    // Parse the input into a 2D vector of integers
    let map: Vec<Vec<u8>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let rows = map.len();
    let cols = map[0].len();

    // Find all trailheads (positions with height 0)
    let mut trailheads = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                trailheads.push((r, c));
            }
        }
    }

    // Function to perform BFS and find all reachable 9s from a given trailhead
    fn bfs(map: &Vec<Vec<u8>>, start: (usize, usize)) -> std::collections::HashSet<(usize, usize)> {
        let rows = map.len();
        let cols = map[0].len();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut reachable_nines = std::collections::HashSet::new();

        // Directions for moving up, down, left, right
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        queue.push_back((start.0, start.1, 0)); // (row, col, current height)
        visited.insert((start.0, start.1));

        while let Some((r, c, current_height)) = queue.pop_front() {
            for (dr, dc) in directions.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if !visited.contains(&(nr, nc)) && map[nr][nc] == current_height + 1 {
                        visited.insert((nr, nc));
                        queue.push_back((nr, nc, map[nr][nc]));

                        if map[nr][nc] == 9 {
                            reachable_nines.insert((nr, nc));
                        }
                    }
                }
            }
        }

        reachable_nines
    }

    // Calculate the score for each trailhead
    let mut total_score = 0;
    for trailhead in trailheads {
        let reachable_nines = bfs(&map, trailhead);
        total_score += reachable_nines.len();
    }

    println!("Total score of all trailheads: {}", total_score);
}

fn main() {
    part_one();
    part_two();
}
