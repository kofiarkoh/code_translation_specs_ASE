use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated integers (matches scanf("%d") behavior on valid input).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // No explicit error handling required by spec.
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        // C uses (n | m) == 0 to terminate; equivalent to n == 0 && m == 0 for non-negative ints.
        if n == 0 && m == 0 {
            break;
        }

        // Fixed 501x501 adjacency matrix and list, as in the C code.
        // Note: Spec mentions potential OOB if endpoints outside 0..500; we mirror the fixed capacity.
        let mut student = vec![vec![0u8; 501]; 501];
        let mut list = vec![0u8; 501];

        for _ in 0..m {
            let p1: usize = it.next().unwrap().parse().unwrap();
            let p2: usize = it.next().unwrap().parse().unwrap();
            // Indexing will panic if out of bounds; C would have UB. Spec says memory safety not guaranteed.
            student[p1][p2] = 1;
            student[p2][p1] = 1;
        }

        // Mark direct neighbors of 1 among vertices 2..=n.
        for i in 2..=n {
            if student[1][i] == 1 {
                list[i] = 1;
            }
        }

        // Mark neighbors-of-neighbors: for each i adjacent to 1, mark all j adjacent to i.
        for i in 2..=n {
            if student[1][i] == 1 {
                for j in 2..=n {
                    if student[i][j] == 1 {
                        list[j] = 1;
                    }
                }
            }
        }

        // Count marked vertices in 2..=n (vertex 1 excluded).
        let mut count: u32 = 0;
        for i in 2..=n {
            count += list[i] as u32;
        }

        out.push_str(&format!("{count}\n"));
    }

    print!("{out}");
}
