use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Fixed-size arrays to mirror the C code's stack arrays and indexing.
    // Note: C code uses indices 0..=500; we keep the same.
    let mut student = [[0i32; 501]; 501];
    let mut list = [0i32; 501];

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        let m_opt = it.next();
        if n_opt.is_none() || m_opt.is_none() {
            break; // No more complete test case; C would have undefined behavior, we stop.
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let m: i32 = m_opt.unwrap().parse().unwrap();

        if (n | m) == 0 {
            break;
        }

        // Clear arrays exactly as in C.
        for i in 0..=500usize {
            list[i] = 0;
            for j in 0..=500usize {
                student[i][j] = 0;
            }
        }

        // Read edges.
        for _ in 0..m {
            let p1: usize = it.next().unwrap().parse().unwrap();
            let p2: usize = it.next().unwrap().parse().unwrap();
            student[p1][p2] = 1;
            student[p2][p1] = 1;
        }

        // Mark direct friends of 1.
        for i in 2..=n {
            let iu = i as usize;
            if student[1][iu] == 1 {
                list[iu] = 1;
            }
        }

        // Mark friends-of-friends via any direct friend of 1.
        for i in 2..=n {
            let iu = i as usize;
            if student[1][iu] == 1 {
                for j in 2..=n {
                    let ju = j as usize;
                    if student[iu][ju] == 1 {
                        list[ju] = 1;
                    }
                }
            }
        }

        // Count marked students excluding 1.
        let mut count: i32 = 0;
        for i in 2..=n {
            count += list[i as usize];
        }

        out.push_str(&format!("{count}\n"));
    }

    print!("{out}");
}
