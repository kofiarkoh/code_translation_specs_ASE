use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut teams: HashMap<i32, i32> = HashMap::new(); // team_id -> score
    let mut input = stdin.lock().lines();

    // Read team-score pairs
    loop {
        let line = input.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let team_id: i32 = parts.next().unwrap().parse().unwrap();
        let score: i32 = parts.next().unwrap().parse().unwrap();

        if team_id == 0 && score == 0 {
            break; // Termination condition
        }

        teams.insert(team_id, score);
    }

    // Create a vector of (team_id, score) pairs and sort them
    let mut sorted_teams: Vec<(i32, i32)> = teams.iter().map(|(&id, &score)| (id, score)).collect();
    sorted_teams.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by score descending

    // Assign ranks
    let mut ranks: HashMap<i32, usize> = HashMap::new();
    let mut current_rank = 1;
    let mut last_score = i32::MIN;

    for (i, &(team_id, score)) in sorted_teams.iter().enumerate() {
        if score != last_score {
            current_rank = i + 1; // Update rank only if score changes
            last_score = score;
        }
        ranks.insert(team_id, current_rank);
    }

    // Process queries
    for line in input {
        let line = line.unwrap();
        let team_id: i32 = line.trim().parse().unwrap();

        // Output the rank if the team_id exists
        if let Some(&rank) = ranks.get(&team_id) {
            println!("{}", rank);
        }
    }
}
