use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut teams = Vec::new();
    let mut scores = Vec::new();
    let mut ranks = Vec::new();
    let mut count = 0;

    // Input
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            continue; // Skip malformed lines
        }
        let team: i32 = parts[0].parse().unwrap();
        let score: i32 = parts[1].parse().unwrap();
        if team + score == 0 {
            break;
        }
        teams.push(team);
        scores.push(score);
        count += 1;
    }

    // Sorting
    for i in 0..count {
        for j in (i + 1..count).rev() {
            if scores[i] <= scores[j] {
                scores.swap(i, j);
                teams.swap(i, j);
            }
        }
    }

    // Ranking
    let mut number = 1;
    ranks.push(number);
    let mut tmp2 = scores[0];
    for i in 1..count {
        if scores[i] != tmp2 {
            number += 1;
            tmp2 = scores[i];
        }
        ranks.push(number);
    }

    // Query
    let mut input = String::new();
    while let Ok(n) = stdin.read_line(&mut input) {
        if n == 0 {
            break; // EOF
        }
        let info: i32 = input.trim().parse().unwrap();
        for i in 0..count {
            if info == teams[i] {
                println!("{}", ranks[i]);
            }
        }
        input.clear(); // Clear input for the next read
    }
}
