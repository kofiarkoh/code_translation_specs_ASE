use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut teams: Vec<(i32, i32)> = Vec::new(); // (team_id, score)
    let mut count = 0;

    // Input processing
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            continue; // Skip invalid input
        }

        let team_id: i32 = match parts[0].trim().parse() {
            Ok(id) => id,
            Err(_) => continue, // Skip invalid input
        };
        let score: i32 = match parts[1].trim().parse() {
            Ok(s) => s,
            Err(_) => continue, // Skip invalid input
        };

        if team_id == 0 && score == 0 {
            break; // Termination condition
        }

        teams.push((team_id, score));
        count += 1;

        if count >= 100 {
            break; // Limit to 100 teams
        }
    }

    // Sorting teams based on scores (non-decreasing order)
    teams.sort_by(|a, b| a.1.cmp(&b.1));

    // Ranking teams
    let mut ranks = vec![0; count];
    let mut current_rank = 1;
    let mut last_score = teams[0].1;

    ranks[0] = current_rank;

    for i in 1..count {
        if teams[i].1 != last_score {
            current_rank += 1;
            last_score = teams[i].1;
        }
        ranks[i] = current_rank;
    }

    // Query processing
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let query: i32 = match line.trim().parse() {
            Ok(q) => q,
            Err(_) => continue, // Skip invalid input
        };

        for i in 0..count {
            if query == teams[i].0 {
                println!("{}", ranks[i]);
                break; // Output the rank and stop searching
            }
        }
    }
}
