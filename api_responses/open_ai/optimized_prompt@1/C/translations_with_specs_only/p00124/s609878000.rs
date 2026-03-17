use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct Team {
    name: String,
    wins: u32,
    draws: u32,
}

impl Team {
    // Calculate the score based on wins and draws
    fn score(&self) -> u32 {
        self.wins * 3 + self.draws
    }
}

fn main() {
    let stdin = io::stdin();
    let mut first_set = true;

    loop {
        // Read the number of teams
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let num_teams: u32 = match input.trim().parse() {
            Ok(n) if n <= 10 => n, // Ensure the number of teams does not exceed 10
            Ok(_) => continue, // Ignore invalid counts greater than 10
            Err(_) => continue, // Undefined behavior for non-integer input
        };

        if num_teams == 0 {
            break; // Terminate gracefully on zero input
        }

        let mut teams: Vec<Team> = Vec::new();

        // Read team data
        for _ in 0..num_teams {
            let mut team_input = String::new();
            stdin.lock().read_line(&mut team_input).unwrap();
            let parts: Vec<&str> = team_input.trim().split_whitespace().collect();

            if parts.len() != 4 {
                continue; // Skip invalid input lines
            }

            let name = parts[0].to_string();
            let wins: u32 = parts[1].parse().unwrap_or(0);
            let draws: u32 = parts[2].parse().unwrap_or(0);
            let _loses: u32 = parts[3].parse().unwrap_or(0); // Loses are not used in score calculation

            teams.push(Team { name, wins, draws });
        }

        // Sort teams by score in descending order, maintaining input order for ties
        teams.sort_by(|a, b| {
            let score_cmp = b.score().cmp(&a.score());
            if score_cmp == Ordering::Equal {
                // Maintain input order for teams with the same score
                Ordering::Equal
            } else {
                score_cmp
            }
        });

        // Output results
        if !first_set {
            println!(); // Print a blank line between sets of results
        }
        first_set = false;

        for team in teams {
            println!("{},{}", team.name, team.score());
        }
    }
}
