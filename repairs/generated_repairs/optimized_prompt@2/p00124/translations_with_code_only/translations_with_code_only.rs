use std::io::{self, BufRead};

#[derive(Debug)]
struct Team {
    name: String,
    score: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock(); // Make reader immutable
    let mut teams: Vec<Team> = Vec::new();
    let mut first_output = false;

    for line in reader.lines() {
        let line = line.unwrap();
        let n: i32 = line.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        teams.clear(); // Clear previous teams

        let mut reader = reader.clone(); // Clone the reader for further use
        for _ in 0..n {
            let mut input = String::new();
            reader.read_line(&mut input).unwrap(); // Use the mutable reader
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let name = parts[0].to_string();
            let win: i32 = parts[1].parse().unwrap();
            let lose: i32 = parts[2].parse().unwrap();
            let draw: i32 = parts[3].parse().unwrap();
            let score = win * 3 + draw;

            teams.push(Team { name, score });
        }

        // Sort teams based on score in descending order
        teams.sort_by(|a, b| b.score.cmp(&a.score));

        if first_output {
            println!(); // Print a blank line before the next output
        }
        for team in &teams {
            println!("{},{}", team.name, team.score);
        }
        first_output = true; // Set flag to indicate output has been printed
    }
}
