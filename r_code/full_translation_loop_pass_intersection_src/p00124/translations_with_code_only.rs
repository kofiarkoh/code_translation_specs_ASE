use std::io::{self, Read};

#[derive(Clone)]
struct Team {
    name: String,
    score: i32,
}

fn main() {
    // Read entire stdin, then token-scan like repeated scanf("%d"/"%s"...).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut f: i32 = 0;

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // No more input; C would block, but for batch input this is fine.
        }
        let n: usize = n_opt.unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        // C has TEAM team[10]; and uses only first n entries.
        // We mirror that with a Vec sized n (assuming input respects n<=10 as in C).
        let mut teams: Vec<Team> = Vec::with_capacity(n);

        for _ in 0..n {
            let name = it.next().unwrap().to_string();
            let win: i32 = it.next().unwrap().parse().unwrap();
            let lose: i32 = it.next().unwrap().parse().unwrap();
            let draw: i32 = it.next().unwrap().parse().unwrap();
            let _ = lose; // present in input but unused in scoring, as in C

            // C uses int arithmetic; in Rust, use i32 and wrapping to emulate overflow behavior.
            let score = win.wrapping_mul(3).wrapping_add(draw);

            teams.push(Team { name, score });
        }

        // Bubble sort descending by score, swapping only when strictly greater (stable for ties),
        // matching the C code's behavior.
        for i in 0..n {
            for j in (i + 1..n).rev() {
                if teams[j].score > teams[j - 1].score {
                    teams.swap(j, j - 1);
                }
            }
        }

        if f == 1 {
            // puts("") prints a newline.
            println!();
        }

        for t in &teams {
            // Exact formatting: "%s,%d\n"
            println!("{},{}", t.name, t.score);
            f = 1;
        }
    }
}
