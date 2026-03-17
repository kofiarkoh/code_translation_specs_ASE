use std::io::{self, Write};

#[derive(Debug)]
struct Team {
    name: String,
    wins: u32,
    losses: u32,
    draws: u32,
}

impl Team {
    // Calculate the score based on wins, draws, and losses
    fn score(&self) -> u32 {
        self.wins * 3 + self.draws // Each win is worth 3 points, each draw is worth 1 point
    }
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: u32 = input.trim().parse().expect("Please enter a valid number");

        // Termination condition
        if n == 0 {
            break;
        }

        // Ensure n is within the valid range
        if n < 1 || n > 10 {
            eprintln!("Number of teams must be between 1 and 10.");
            continue;
        }

        let mut teams: Vec<Team> = Vec::new();

        for _ in 0..n {
            let mut team_input = String::new();
            io::stdin().read_line(&mut team_input).expect("Failed to read line");
            let parts: Vec<&str> = team_input.trim().split_whitespace().collect();

            // Expecting input format: name wins losses draws
            if parts.len() != 4 {
                eprintln!("Invalid input format. Expected: name wins losses draws");
                continue;
            }

            let name = parts[0].to_string();
            let wins: u32 = parts[1].parse().expect("Wins must be a non-negative integer");
            let losses: u32 = parts[2].parse().expect("Losses must be a non-negative integer");
            let draws: u32 = parts[3].parse().expect("Draws must be a non-negative integer");

            teams.push(Team { name, wins, losses, draws });
        }

        // Sort teams by score in descending order
        teams.sort_by(|a, b| b.score().cmp(&a.score()));

        // Output the results
        for team in teams {
            println!("{},{}", team.name, team.score());
        }
    }
}
