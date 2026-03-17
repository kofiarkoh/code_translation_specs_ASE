use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of items
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Initialize a set to track item positions
    let mut items = HashSet::new();
    
    // Read item positions
    for _ in 0..n {
        let coords: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        items.insert((coords[0], coords[1]));
    }

    // Read number of moves
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initial player position
    let mut player_x = 0;
    let mut player_y = 0;

    // Process moves
    for _ in 0..m {
        let move_info: Vec<&str> = lines.next().unwrap().unwrap().split_whitespace().collect();
        let direction = move_info[0].chars().next().unwrap();
        let steps: usize = move_info[1].parse().unwrap();

        // Move the player in the specified direction
        for _ in 0..steps {
            match direction {
                'N' => if player_y > 0 { player_y -= 1; },
                'E' => if player_x < 24 { player_x += 1; },
                'S' => if player_y < 24 { player_y += 1; },
                'W' => if player_x > 0 { player_x -= 1; },
                _ => unreachable!(), // Direction should always be valid as per specification
            }

            // Collect item if present at the new position
            items.remove(&(player_x, player_y));
        }
    }

    // Output "Yes" if all items have been collected, otherwise "No"
    if items.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
