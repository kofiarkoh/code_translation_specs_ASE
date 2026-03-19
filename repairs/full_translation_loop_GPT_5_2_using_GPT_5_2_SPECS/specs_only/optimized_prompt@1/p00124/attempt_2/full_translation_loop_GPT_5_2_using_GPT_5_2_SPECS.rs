use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone)]
struct Team {
    name: String,
    points: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut printed_any_dataset = false;

    loop {
        let n_tok = match it.next() {
            Some(t) => t,
            None => break, // stop if input ends unexpectedly
        };
        let n: usize = n_tok.parse().unwrap();
        if n == 0 {
            break;
        }

        let mut teams: Vec<Team> = Vec::with_capacity(n);
        for _ in 0..n {
            let name = it.next().unwrap().to_string();
            let wins: i32 = it.next().unwrap().parse().unwrap();
            let _losses: i32 = it.next().unwrap().parse().unwrap();
            let draws: i32 = it.next().unwrap().parse().unwrap();
            let points = wins * 3 + draws;
            teams.push(Team { name, points });
        }

        // Order by descending points; ties need not preserve input order.
        teams.sort_unstable_by(|a, b| match b.points.cmp(&a.points) {
            Ordering::Equal => Ordering::Equal,
            other => other,
        });

        if printed_any_dataset {
            print!("\n");
        }
        printed_any_dataset = true;

        // Output format: "TeamName,Points"
        for t in teams {
            println!("{},{}", t.name, t.points);
        }
    }
}
