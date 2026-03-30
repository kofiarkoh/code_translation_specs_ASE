use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct T {
    stu: usize,
    seq: usize,
    id: usize,
}

fn cmp(a: &T, b: &T) -> Ordering {
    match a.seq.cmp(&b.seq) {
        Ordering::Equal => a.stu.cmp(&b.stu),
        other => other,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut req: Vec<T> = Vec::new();
    let mut hi = vec![0; 12];
    let mut book = vec![vec![0; 2]; 101];
    let mut desk = vec![vec![0; 5001]; 12];

    loop {
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let m: usize = match iter.next() {
            Some(num) => num.parse().unwrap(),
            None => break,
        };
        let c: usize = iter.next().unwrap().parse().unwrap();
        let n: usize = iter.next().unwrap().parse().unwrap();

        if m == 0 {
            break;
        }

        hi.fill(0);
        book.iter_mut().for_each(|b| b.fill(0));
        let tmp = m + 1;
        let mut len = 0;

        for i in 0..n {
            let t: usize = input.read_line(&mut line).unwrap().trim().parse().unwrap();
            let ids: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            for j in 0..t {
                let id = ids[j];
                req.push(T { stu: i, seq: j, id });
                if book[id][0] == 0 {
                    book[id][0] = tmp;
                    book[id][1] = hi[tmp];
                    desk[tmp][hi[tmp]] = id;
                    hi[tmp] += 1;
                }
            }
        }

        req.sort_by(cmp);

        let mut ans = 0;
        for i in 0..len {
            let id = req[i].id;
            let from = book[id][0];
            let pos = book[id][1];

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
                let mut pos = hi[j];
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
