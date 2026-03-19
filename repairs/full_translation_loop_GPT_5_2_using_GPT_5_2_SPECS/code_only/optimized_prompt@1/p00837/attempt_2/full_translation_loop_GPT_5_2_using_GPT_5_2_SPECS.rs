use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Req {
    stu: i32,
    seq: i32,
    id: usize,
}

fn main() {
    // Fast integer parser (avoids split_whitespace overhead that can cause TLE).
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut idx = 0usize;

    fn next_usize(a: &[u8], idx: &mut usize) -> Option<usize> {
        while *idx < a.len() && a[*idx].is_ascii_whitespace() {
            *idx += 1;
        }
        if *idx >= a.len() {
            return None;
        }
        let mut sign = 1i32;
        if a[*idx] == b'-' {
            sign = -1;
            *idx += 1;
        }
        let mut v: usize = 0;
        while *idx < a.len() {
            let c = a[*idx];
            if !c.is_ascii_digit() {
                break;
            }
            v = v * 10 + (c - b'0') as usize;
            *idx += 1;
        }
        if sign < 0 {
            // Not expected for this problem, but keep behavior defined.
            // Clamp to 0 in usize domain.
            Some(0)
        } else {
            Some(v)
        }
    }

    let mut out = String::new();

    loop {
        let m = match next_usize(&input, &mut idx) {
            Some(v) => v,
            None => break,
        };
        let c = next_usize(&input, &mut idx).unwrap();
        let n = next_usize(&input, &mut idx).unwrap();

        if m == 0 {
            break;
        }

        // Fixed-size arrays as in C.
        let mut req: Vec<Req> = Vec::with_capacity(5002);
        let mut desk = vec![vec![0usize; 5001]; 12];
        let mut hi = vec![0usize; 12];
        let mut book = vec![[0usize; 2]; 101];

        let tmp_init = m + 1; // initial desk index for first-time-seen books

        for i in 0..n {
            let t = next_usize(&input, &mut idx).unwrap();
            for j in 0..t {
                let id = next_usize(&input, &mut idx).unwrap();
                req.push(Req {
                    stu: i as i32,
                    seq: j as i32,
                    id,
                });
                if book[id][0] == 0 {
                    book[id][0] = tmp_init;
                    book[id][1] = hi[tmp_init];
                    desk[tmp_init][hi[tmp_init]] = id;
                    hi[tmp_init] += 1;
                }
            }
        }

        req.sort_by(|a, b| match a.seq.cmp(&b.seq) {
            Ordering::Equal => a.stu.cmp(&b.stu),
            other => other,
        });

        let mut ans: i64 = 0;

        for r in req.iter() {
            let id = r.id;
            let from = book[id][0];
            let pos = book[id][1];

            // Remove id from desk[from] at index pos by shifting left.
            // Use slice copy for speed; also update book[*][1] for shifted items.
            if pos + 1 < hi[from] {
                for j in (pos + 1)..hi[from] {
                    let t_id = desk[from][j];
                    book[t_id][1] -= 1;
                }
                // shift desk[from][pos+1..hi[from]] left by 1
                let src_range = (pos + 1)..hi[from];
                let dst_start = pos;
                desk[from].copy_within(src_range, dst_start);
            }
            hi[from] -= 1;
            ans += from as i64;

            if hi[1] == c {
                // Find first tmp in [2..=m] with hi[tmp] != c
                let mut tmp = 2usize;
                while tmp <= m && hi[tmp] == c {
                    tmp += 1;
                }

                desk[tmp][hi[tmp]] = id;
                hi[tmp] += 1;
                ans += tmp as i64;

                // Evict id2 = desk[1][0] from desk[1] by shifting left.
                let id2 = desk[1][0];
                if hi[1] > 1 {
                    for j in 1..hi[1] {
                        let t_id = desk[1][j];
                        book[t_id][1] -= 1;
                    }
                    desk[1].copy_within(1..hi[1], 0);
                }
                hi[1] -= 1;
                ans += 1;

                // Find first j in [2..=m] with hi[j] != c
                let mut jdesk = 2usize;
                while jdesk <= m && hi[jdesk] == c {
                    jdesk += 1;
                }

                let mut pos2 = hi[jdesk];
                if jdesk == tmp {
                    pos2 -= 1;
                }
                desk[jdesk][pos2] = id2;
                hi[jdesk] += 1;
                book[id2][0] = jdesk;
                book[id2][1] = pos2;

                ans += (jdesk + tmp) as i64;
                hi[tmp] -= 1;
            }

            // Put id onto desk 1 at the end.
            book[id][0] = 1;
            book[id][1] = hi[1];
            desk[1][hi[1]] = id;
            hi[1] += 1;
            ans += 1;
        }

        out.push_str(&ans.to_string());
        out.push('\n');
    }

    print!("{out}");
}
