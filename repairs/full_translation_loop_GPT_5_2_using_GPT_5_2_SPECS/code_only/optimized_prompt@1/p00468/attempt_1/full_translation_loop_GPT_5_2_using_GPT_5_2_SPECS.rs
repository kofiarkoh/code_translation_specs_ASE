use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // No more input
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();

        let m_opt = it.next();
        if m_opt.is_none() {
            break; // Incomplete pair; C would have undefined behavior, we stop.
        }
        let m: i32 = m_opt.unwrap().parse().unwrap();

        if (n | m) == 0 {
            break;
        }

        // C arrays: int student[501][501]; int list[501];
        // Initialize to 0 each test case, matching the C loops.
        let mut student = vec![vec![0i32; 501]; 501];
        let mut list = vec![0i32; 501];

        for _ in 0..m {
            let p1: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;
            let p2: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;
            // Assume inputs are within 0..=500 as in the C code's intended use.
            student[p1][p2] = 1;
            student[p2][p1] = 1;
        }

        // Direct friends of 1
        for i in 2..=(n as usize) {
            if student[1][i] == 1 {
                list[i] = 1;
            }
        }

        // Friends of friends via any direct friend of 1
        for i in 2..=(n as usize) {
            if student[1][i] == 1 {
                for j in 2..=(n as usize) {
                    if student[i][j] == 1 {
                        list[j] = 1;
                    }
                }
            }
        }

        let mut count: i32 = 0;
        for i in 2..=(n as usize) {
            count += list[i];
        }

        print!("{}\n", count);
    }
}
