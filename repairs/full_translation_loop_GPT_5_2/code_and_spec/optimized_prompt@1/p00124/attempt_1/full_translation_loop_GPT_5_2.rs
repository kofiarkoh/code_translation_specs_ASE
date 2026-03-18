use std::io::{self, Read};

#[derive(Clone)]
struct Team {
    name: String,
    score: i32,
    idx: usize, // to preserve input order on ties (stable tie-break)
}

fn main() {
    // Read entire stdin and tokenize by whitespace (matches scanf("%d") / scanf("%s ..."))
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut first_output = true;

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // no more input
        }
        let n: usize = n_opt.unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut teams: Vec<Team> = Vec::with_capacity(n);

        for i in 0..n {
            let name = it.next().unwrap().to_string();
            let win: i32 = it.next().unwrap().parse().unwrap();
            let _lose: i32 = it.next().unwrap().parse().unwrap(); // present in input but unused
            let draw: i32 = it.next().unwrap().parse().unwrap();

            let score = win * 3 + draw;
            teams.push(Team { name, score, idx: i });
        }

        // Sort by descending score; ties resolved by input order.
        teams.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.idx.cmp(&b.idx)));

        if !first_output {
            println!();
        }
        first_output = false;

        for t in teams {
            println!("{},{}", t.name, t.score);
        }
    }
}
