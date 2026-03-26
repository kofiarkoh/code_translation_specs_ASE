use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct Team {
    name: String,
    score: i32,
}

impl Team {
    fn new(name: String, wins: i32, draws: i32) -> Self {
        // Score is calculated as: score = wins * 3 + draws
        let score = wins * 3 + draws;
        Team { name, score }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut teams: Vec<Team> = Vec::new();

    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        // Termination condition
        if n == 0 {
            break;
        }

        // Read team data
        for _ in 0..n {
            let mut team_input = String::new();
            stdin.lock().read_line(&mut team_input).unwrap();
            let parts: Vec<&str> = team_input.trim().split_whitespace().collect();
            let name = parts[0].to_string();
            let wins: i32 = parts[1].parse().unwrap();
            let losses: i32 = parts[2].parse().unwrap(); // Not used in score calculation
            let draws: i32 = parts[3].parse().unwrap();

            // Create a new team and add it to the list
            teams.push(Team::new(name, wins, draws));
        }

        // Sort teams by score in descending order
        teams.sort_by(|a, b| b.score.cmp(&a.score));

        // Output the results
        for team in &teams {
            println!("{},{}", team.name, team.score);
        }

        // Clear the teams for the next iteration
        teams.clear();
    }
}
