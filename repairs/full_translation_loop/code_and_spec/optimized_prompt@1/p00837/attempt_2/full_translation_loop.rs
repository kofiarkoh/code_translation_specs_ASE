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
    let input = stdin.lock();
    let mut line = String::new();

    for line in input.lines() {
        let line = line.unwrap();
        let params: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let m = params[0];
        let c = params[1];
        let n = params[2];

        if m == 0 || c == 0 || n == 0 {
            break; // Terminate if any input is non-positive
        }

        let mut req: Vec<Request> = Vec::new();
        let mut hi = vec![0; m + 1];
        let mut book = vec![vec![0; 2]; 101];
        let mut desk = vec![vec![0; 5001]; 12];

        for i in 0..n {
            line.clear();
            input.read_line(&mut line).unwrap();
            let t: usize = line.trim().parse().unwrap();

            for j in 0..t {
                line.clear();
                input.read_line(&mut line).unwrap();
                let id: usize = line.trim().parse().unwrap();
                req.push(Request { stu: i, seq: j, id });

                if book[id][0] == 0 {
                    book[id][0] = m + 1; // Assign a new desk
                    book[id][1] = hi[book[id][0]];
                    desk[book[id][0]][hi[book[id][0]]] = id;
                    hi[book[id][0]] += 1;
                }
            }
        }

        req.sort_by(|a, b| {
            match a.seq.cmp(&b.seq) {
                Ordering::Equal => a.stu.cmp(&b.stu),
                other => other,
            }
        });

        let mut ans = 0;

        for request in req {
            let id = request.id;
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
                let mut pos = hi[j]; // Make pos mutable
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
