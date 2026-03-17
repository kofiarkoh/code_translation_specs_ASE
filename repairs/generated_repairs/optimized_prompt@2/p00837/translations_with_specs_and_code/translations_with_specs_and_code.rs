// AOJ 1258: Book Replacement
// Translated from C to Rust

use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct Request {
    stu: usize,
    seq: usize,
    id: usize,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut params = line.split_whitespace().map(|s| s.parse::<usize>());
        let m = params.next().unwrap().unwrap();
        let c = params.next().unwrap().unwrap();
        let n = params.next().unwrap().unwrap();

        if m == 0 || c == 0 || n == 0 {
            break; // Terminate if any input is non-positive
        }

        let mut req: Vec<Request> = Vec::new();
        let mut desk: Vec<Vec<usize>> = vec![vec![0; 5001]; 12];
        let mut hi: Vec<usize> = vec![0; 12];
        let mut book: Vec<[usize; 2]> = vec![[0; 2]; 101];
        let mut len = 0;

        for i in 0..n {
            let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
            let ids: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok()).collect();

            for j in 0..t {
                let id = ids[j];
                req.push(Request { stu: i, seq: j, id });
                if book[id][0] == 0 {
                    book[id][0] = m + 1; // Use m + 1 as a temporary desk number
                    book[id][1] = hi[book[id][0]];
                    desk[book[id][0]][hi[book[id][0]]] = id;
                    hi[book[id][0]] += 1;
                }
                len += 1;
            }
        }

        req.sort_by(|a, b| {
            match a.seq.cmp(&b.seq) {
                Ordering::Equal => a.stu.cmp(&b.stu),
                other => other,
            }
        });

        let mut ans = 0;

        for i in 0..len {
            let id = req[i].id;
            let from = book[id][0];
            let pos = book[id][1];

            // Shift books on the desk
            for j in (pos + 1)..hi[from] {
                let t = desk[from][j];
                book[t][1] -= 1;
                desk[from][j - 1] = desk[from][j];
            }
            hi[from] -= 1;
            ans += from;

            if hi[1] == c {
                let mut tmp = 2;
                while tmp <= m && hi[tmp] == c {
                    tmp += 1;
                }
                desk[tmp][hi[tmp]] = id;
                hi[tmp] += 1;
                ans += tmp;

                let id2 = desk[1][0];
                for j in 1..hi[1] {
                    let t = desk[1][j];
                    book[t][1] -= 1;
                    desk[1][j - 1] = desk[1][j];
                }
                hi[1] -= 1;
                ans += 1;

                let mut j = 2;
                while j <= m && hi[j] == c {
                    j += 1;
                }
                let mut pos = hi[j]; // Change to mutable variable
                if j == tmp {
                    pos -= 1;
                }
                desk[j][pos] = id2;
                hi[j] += 1;
                book[id2][0] = j;
                book[id2][1] = pos;
                ans += j + tmp;
                hi[tmp] -= 1;
            }

            book[id][0] = 1;
            book[id][1] = hi[1];
            desk[1][hi[1]] = id;
            hi[1] += 1;
            ans += 1;
        }
        println!("{}", ans);
    }
}
