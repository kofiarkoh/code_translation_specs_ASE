use std::io::{self, Read};

#[derive(Clone)]
struct Team {
    name: String,
    score: i32,
}

fn main() {
    // Read entire stdin and tokenize like repeated scanf("%d"/"%s"...).
    // This matches C's whitespace-delimited scanning behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut f: i32 = 0;

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // If input ends unexpectedly, just stop (C would block; best-effort).
        }
        let n: usize = n_opt.unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut teams: Vec<Team> = Vec::with_capacity(n);

        for _ in 0..n {
            let name = it.next().unwrap().to_string();
            let win: i32 = it.next().unwrap().parse().unwrap();
            let _lose: i32 = it.next().unwrap().parse().unwrap();
            let draw: i32 = it.next().unwrap().parse().unwrap();

            // In C, int overflow is UB; we use wrapping to emulate typical 2's-complement behavior.
            let score = win.wrapping_mul(3).wrapping_add(draw);

            teams.push(Team { name, score });
        }

        // Bubble sort descending by score, swapping only when strictly greater.
        // This preserves the C code's stability for equal scores.
        for i in 0..n {
            for j in (i + 1..n).rev() {
                if teams[j].score > teams[j - 1].score {
                    teams.swap(j, j - 1);
                }
            }
        }

        if f == 1 {
            print!("\n");
        }

        for team in &teams {
            // Match C: "%s,%d\n"
            println!("{},{}", team.name, team.score);
            f = 1;
        }
    }
}
