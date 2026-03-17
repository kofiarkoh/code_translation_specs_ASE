use std::io::{self, BufRead};
use std::collections::VecDeque;

#[derive(Debug)]
struct Team {
    name: String,
    score: i32,
}

impl Team {
    // Calculate the score based on wins and draws
    fn new(name: String, wins: i32, draws: i32) -> Self {
        let score = wins * 3 + draws; // Score calculation as per specification
        Team { name, score }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut teams: VecDeque<Team>;

    loop {
        // Read the number of teams
        let mut input = String::new();
        reader.read_line(&mut input).expect("Failed to read line");
        let num_teams: i32 = input.trim().parse().expect("Invalid input"); // Undefined behavior on non-integer

        if num_teams == 0 {
            break; // Terminate gracefully on zero input
        }

        // Ensure we do not exceed the maximum number of teams
        if num_teams < 0 || num_teams > 10 {
            continue; // Ignore invalid team counts
        }

        teams = VecDeque::new();

        // Read team data
        for _ in 0..num_teams {
            let mut team_input = String::new();
            reader.read_line(&mut team_input).expect("Failed to read line");
            let parts: Vec<&str> = team_input.trim().split_whitespace().collect();

            // Expecting format: "name wins draws"
            if parts.len() != 3 {
                continue; // Ignore invalid input lines
            }

            let name = parts[0].to_string();
            let wins: i32 = parts[1].parse().expect("Invalid input"); // Undefined behavior on non-integer
            let draws: i32 = parts[2].parse().expect("Invalid input"); // Undefined behavior on non-integer

            // Create a new team and add it to the list
            teams.push_back(Team::new(name, wins, draws));
        }

        // Sort teams by score in descending order
        let mut sorted_teams: Vec<Team> = teams.into_iter().collect();
        sorted_teams.sort_by(|a, b| b.score.cmp(&a.score));

        // Output results
        for team in sorted_teams {
            println!("{},{}", team.name, team.score);
        }
        println!(); // Blank line between sets of results
    }
}
