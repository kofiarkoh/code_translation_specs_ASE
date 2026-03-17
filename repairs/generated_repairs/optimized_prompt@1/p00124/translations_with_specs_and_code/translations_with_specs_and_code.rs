use std::io::{self, BufRead};

#[derive(Debug)]
struct Team {
    name: String,
    score: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Make reader mutable

    let mut first_output = false;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let n: i32 = line.trim().parse().expect("Invalid input for number of teams");

        if n == 0 {
            break;
        }

        let mut teams: Vec<Team> = Vec::new();

        for _ in 0..n {
            let mut team_input = String::new();
            reader.read_line(&mut team_input).expect("Failed to read team data");
            let parts: Vec<&str> = team_input.trim().split_whitespace().collect();

            let name = parts[0].to_string();
            let wins: i32 = parts[1].parse().expect("Invalid input for wins");
            let draws: i32 = parts[2].parse().expect("Invalid input for draws"); // Fixed index for draws

            let score = wins * 3 + draws;

            teams.push(Team { name, score });
        }

        // Sort teams by score in descending order, maintaining input order for ties
        teams.sort_by(|a, b| b.score.cmp(&a.score));

        if first_output {
            println!(); // Print a blank line before the next output
        }
        for team in teams {
            println!("{},{}", team.name, team.score);
        }
        first_output = true;
    }
}
