use std::io::{self, Write};

#[derive(Debug)]
struct Team {
    name: String,
    score: i32,
}

fn main() {
    let mut input = String::new();
    let mut f = false;

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n == 0 {
            break;
        }

        let mut teams: Vec<Team> = Vec::with_capacity(n);
        
        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let name = parts[0].to_string();
            let win: i32 = parts[1].parse().unwrap();
            let lose: i32 = parts[2].parse().unwrap();
            let draw: i32 = parts[3].parse().unwrap();
            let score = win * 3 + draw;

            teams.push(Team { name, score });
        }

        // Bubble sort to sort teams by score in descending order
        for i in 0..n {
            for j in (i + 1..n).rev() {
                if teams[j].score > teams[j - 1].score {
                    teams.swap(j, j - 1);
                }
            }
        }

        if f {
            println!(); // Print a newline if this is not the first output
        }
        for team in teams {
            println!("{},{}", team.name, team.score);
            f = true; // Set f to true after the first output
        }
    }
}
