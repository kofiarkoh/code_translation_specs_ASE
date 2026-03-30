use std::io::{self, Write};

#[derive(Debug)]
struct Team {
    name: String,
    score: i32,
}

fn main() {
    let mut input = String::new();
    let mut first_set = true; // Track if it's the first set of teams

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: i32 = input.trim().parse().expect("Invalid input");

        if n == 0 {
            break;
        }

        // Ensure n is within the valid range
        if n < 1 || n > 10 {
            continue; // Undefined behavior as per spec; we skip invalid cases
        }

        let mut teams: Vec<Team> = Vec::new();

        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let name = parts[0].to_string();
            let wins: i32 = parts[1].parse().expect("Invalid input");
            let losses: i32 = parts[2].parse().expect("Invalid input");
            let draws: i32 = parts[3].parse().expect("Invalid input");

            let score = wins * 3 + draws;
            teams.push(Team { name, score });
        }

        // Sort teams by score in descending order
        teams.sort_by(|a, b| b.score.cmp(&a.score));

        // Print results with a blank line between sets
        if !first_set {
            println!(); // Print a blank line before the output if not the first set
        }
        first_set = false; // After the first set, we set this to false

        for team in teams {
            println!("{},{}", team.name, team.score);
        }
    }
}
