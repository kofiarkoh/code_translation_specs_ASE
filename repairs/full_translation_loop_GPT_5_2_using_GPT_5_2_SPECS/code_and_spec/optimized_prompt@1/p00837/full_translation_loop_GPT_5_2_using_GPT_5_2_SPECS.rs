use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Req {
    stu: i32,
    seq: i32,
    id: usize,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let Some(m_tok) = it.next() else { break };
        let m: i32 = m_tok.parse().unwrap();
        let c: i32 = it.next().unwrap().parse().unwrap();
        let n: i32 = it.next().unwrap().parse().unwrap();

        if m <= 0 {
            break;
        }

        // Mirror the C program's fixed bounds:
        // desks: [12][5001], hi: [12], book: [101][2], req: [5002]
        // Desk indices used: 1..=m+1 (1 primary, 2..=m auxiliary, m+1 initial storage).
        let mut desk: Vec<Vec<usize>> = vec![Vec::new(); 12];
        let mut hi: Vec<usize> = vec![0; 12];

        let mut book_desk: Vec<usize> = vec![0; 101];
        let mut book_pos: Vec<usize> = vec![0; 101];

        let mut reqs: Vec<Req> = Vec::new();

        let initial_desk = (m + 1) as usize;

        for stu in 0..n {
            let t: i32 = it.next().unwrap().parse().unwrap();
            for seq in 0..t {
                let id: usize = it.next().unwrap().parse().unwrap();
                reqs.push(Req { stu, seq, id });

                if book_desk[id] == 0 {
                    book_desk[id] = initial_desk;
                    book_pos[id] = hi[initial_desk];
                    // keep desk vectors in sync with hi
                    if desk[initial_desk].len() == hi[initial_desk] {
                        desk[initial_desk].push(id);
                    } else {
                        desk[initial_desk][hi[initial_desk]] = id;
                    }
                    hi[initial_desk] += 1;
                }
            }
        }

        reqs.sort_by(|a, b| match a.seq.cmp(&b.seq) {
            Ordering::Equal => a.stu.cmp(&b.stu),
            other => other,
        });

        let cap = c as usize;
        let m_us = m as usize;

        let mut ans: i64 = 0;

        for r in reqs {
            let id = r.id;
            let from = book_desk[id];
            let pos = book_pos[id];

            // Remove id from desk[from] at index pos, compacting left.
            for j in (pos + 1)..hi[from] {
                let t_id = desk[from][j];
                book_pos[t_id] -= 1;
                desk[from][j - 1] = t_id;
            }
            hi[from] -= 1;
            // keep vector length >= hi; no need to pop, we use hi as authoritative
            ans += from as i64;

            if hi[1] == cap {
                // Find first non-full auxiliary desk (2..=m).
                let mut tmp = 2usize;
                while tmp <= m_us && hi[tmp] == cap {
                    tmp += 1;
                }
                // Place requested book temporarily onto tmp.
                if desk[tmp].len() == hi[tmp] {
                    desk[tmp].push(id);
                } else {
                    desk[tmp][hi[tmp]] = id;
                }
                hi[tmp] += 1;
                ans += tmp as i64;

                // Remove the first book from primary desk.
                let id2 = desk[1][0];
                for j in 1..hi[1] {
                    let t_id = desk[1][j];
                    book_pos[t_id] -= 1;
                    desk[1][j - 1] = t_id;
                }
                hi[1] -= 1;
                ans += 1; // cost for interacting with desk 1 (removal)

                // Find first non-full auxiliary desk again.
                let mut j = 2usize;
                while j <= m_us && hi[j] == cap {
                    j += 1;
                }

                // Compute insertion position exactly like C:
                // pos = hi[j]; if (j == tmp) pos--;
                let mut pos2 = hi[j];
                if j == tmp {
                    pos2 -= 1;
                }

                // Place id2 at pos2 (may overwrite the temporary id if j==tmp).
                if desk[j].len() <= pos2 {
                    desk[j].resize(pos2 + 1, 0);
                }
                desk[j][pos2] = id2;
                hi[j] += 1;

                book_desk[id2] = j;
                book_pos[id2] = pos2;

                // Costs: ans += j + tmp; then hi[tmp]-- (remove temporary requested book)
                ans += (j + tmp) as i64;
                hi[tmp] -= 1;
            }

            // Place requested book onto primary desk.
            book_desk[id] = 1;
            book_pos[id] = hi[1];
            if desk[1].len() == hi[1] {
                desk[1].push(id);
            } else {
                desk[1][hi[1]] = id;
            }
            hi[1] += 1;
            ans += 1;
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
