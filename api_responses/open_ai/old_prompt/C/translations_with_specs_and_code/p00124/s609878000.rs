use std::io::{self, BufRead};

#[derive(Debug)]
struct Team {
    name: String,
    score: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: i32 = line.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut teams: Vec<Team> = Vec::new();

        for _ in 0..n {
            let mut input = String::new();
            reader.read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let name = parts[0].to_string();
            let win: i32 = parts[1].parse().unwrap();
            let draw: i32 = parts[2].parse().unwrap();
            let score = win * 3 + draw;

            teams.push(Team { name, score });
        }

        // Sort teams by score in descending order
        teams.sort_by(|a, b| b.score.cmp(&a.score));

        // Print results
        for (i, team) in teams.iter().enumerate() {
            if i > 0 {
                println!(); // Print a blank line before the next output if it's not the first
            }
            println!("{},{}", team.name, team.score);
        }
    }
}
