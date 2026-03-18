use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Req {
    stu: i32,
    seq: i32,
    id: usize,
}

fn main() {
    // Read entire stdin, parse as integers like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let m_opt = it.next();
        if m_opt.is_none() {
            break;
        }
        let m: i32 = m_opt.unwrap().parse().unwrap();
        let c: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        let n: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };

        if m <= 0 {
            break;
        }

        // Arrays sized to match C code bounds.
        // desk[12][5001], hi[12], book[101][2], req[5002]
        let mut req: Vec<Req> = Vec::with_capacity(5002);
        let mut desk = vec![vec![0usize; 5001]; 12];
        let mut hi = vec![0i32; 12];
        let mut book = vec![[0i32; 2]; 101];

        let mut tmp = m + 1; // may be > m; C code uses desk[tmp] with tmp=m+1
        // Ensure desk/hi can index tmp (m+1). C code assumes m <= 10 so tmp <= 11.
        // We keep fixed size 12, matching the original.

        for i in 0..n {
            let t: i32 = it.next().unwrap().parse().unwrap();
            for j in 0..t {
                let id_i32: i32 = it.next().unwrap().parse().unwrap();
                let id = id_i32 as usize;

                req.push(Req {
                    stu: i,
                    seq: j,
                    id,
                });

                if book[id][0] == 0 {
                    book[id][0] = tmp;
                    book[id][1] = hi[tmp as usize];
                    let h = hi[tmp as usize] as usize;
                    desk[tmp as usize][h] = id;
                    hi[tmp as usize] += 1;
                }
            }
        }

        // qsort with cmp: by seq, then by stu (both ascending).
        // In C, cmp returns (a->seq - b->seq) which can overflow; here values are small.
        req.sort_by(|a, b| {
            match a.seq.cmp(&b.seq) {
                Ordering::Equal => a.stu.cmp(&b.stu),
                other => other,
            }
        });

        let mut ans: i32 = 0;

        for r in req.iter() {
            let id = r.id;
            let from = book[id][0] as usize;
            let pos = book[id][1] as usize;

            // Remove id from desk[from] at index pos by shifting left.
            let hi_from = hi[from] as usize;
            for j in (pos + 1)..hi_from {
                let t_id = desk[from][j];
                book[t_id][1] -= 1;
                desk[from][j - 1] = desk[from][j];
            }
            hi[from] -= 1;
            ans += from as i32;

            if hi[1] == c {
                // Find first tmp in [2..=m] with hi[tmp] != c
                let mut tmp2: i32 = 2;
                while tmp2 <= m && hi[tmp2 as usize] == c {
                    tmp2 += 1;
                }
                // Place id into desk[tmp2]
                let h = hi[tmp2 as usize] as usize;
                desk[tmp2 as usize][h] = id;
                hi[tmp2 as usize] += 1;
                ans += tmp2;

                // Evict id2 = desk[1][0] from desk[1]
                let id2 = desk[1][0];
                let hi1 = hi[1] as usize;
                for j in 1..hi1 {
                    let t_id = desk[1][j];
                    book[t_id][1] -= 1;
                    desk[1][j - 1] = desk[1][j];
                }
                hi[1] -= 1;
                ans += 1;

                // Find first j in [2..=m] with hi[j] != c
                let mut jdesk: i32 = 2;
                while jdesk <= m && hi[jdesk as usize] == c {
                    jdesk += 1;
                }
                let mut pos2 = hi[jdesk as usize] as i32;
                if jdesk == tmp2 {
                    pos2 -= 1;
                }
                let pos2u = pos2 as usize;
                desk[jdesk as usize][pos2u] = id2;
                hi[jdesk as usize] += 1;
                book[id2][0] = jdesk;
                book[id2][1] = pos2;

                ans += jdesk + tmp2;
                hi[tmp2 as usize] -= 1;
            }

            // Finally, put id onto desk[1]
            book[id][0] = 1;
            book[id][1] = hi[1];
            let h1 = hi[1] as usize;
            desk[1][h1] = id;
            hi[1] += 1;
            ans += 1;
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
