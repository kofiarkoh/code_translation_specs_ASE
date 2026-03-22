use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // No more input; C code would have undefined behavior, but we stop cleanly.
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let m: i32 = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        let n_usize = n as usize;
        let m_usize = m as usize;

        // q[n][m]
        let mut q = vec![vec![0i32; m_usize]; n_usize];
        for j in 0..n_usize {
            for i in 0..m_usize {
                q[j][i] = it.next().unwrap().parse().unwrap();
            }
        }

        // cnt[m], computed with 32-bit signed wraparound to emulate typical C int overflow behavior.
        let mut cnt = vec![0i32; m_usize];
        for i in 0..n_usize {
            for j in 0..m_usize {
                cnt[j] = cnt[j].wrapping_add(q[i][j]);
            }
        }

        // Selection of indices in descending cnt order, tie-broken by first encountered (smallest j),
        // matching the C logic.
        let mut maxed = vec![false; m_usize];
        let mut out = String::new();

        for i in 0..m_usize {
            if i != 0 {
                out.push(' ');
            }
            let mut max_idx: Option<usize> = None;
            for j in 0..m_usize {
                if !maxed[j] {
                    match max_idx {
                        None => max_idx = Some(j),
                        Some(mi) => {
                            if cnt[mi] < cnt[j] {
                                max_idx = Some(j);
                            }
                        }
                    }
                }
            }
            let max = max_idx.unwrap();
            out.push_str(&(max as i32 + 1).to_string());
            maxed[max] = true;
        }
        out.push('\n');
        print!("{out}");
    }
}
