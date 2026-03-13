use std::io::{self, Read};

fn main() {
    // Read entire stdin and split by whitespace, to emulate scanf("%d %d", ...) and scanf("%s", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut it = input.split_whitespace();

    loop {
        // Parse H and W. If missing, stop.
        let h_token = match it.next() {
            Some(t) => t,
            None => break,
        };
        let w_token = match it.next() {
            Some(t) => t,
            None => break,
        };
        let h: usize = match h_token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let w: usize = match w_token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if h == 0 && w == 0 {
            break;
        }

        // In the C code, farm is a global char[MAX][MAX] zero-initialized.
        // scanf("%s", farm[i]) writes the string and a null terminator into the row;
        // positions beyond the string remain zero ('\0'). To emulate that, we
        // initialize each row with 0 bytes and then copy the input token bytes
        // into the first columns (up to W).
        let mut farm: Vec<Vec<u8>> = Vec::with_capacity(h);
        for _ in 0..h {
            let tok = match it.next() {
                Some(t) => t,
                None => "", // if missing, treat as empty string (but well-formed inputs should provide it)
            };
            let mut row = vec![0u8; w];
            let bytes = tok.as_bytes();
            let limit = std::cmp::min(w, bytes.len());
            row[..limit].copy_from_slice(&bytes[..limit]);
            farm.push(row);
        }

        // Flood-fill counting. The original C code uses recursion and marks visited cells with '+'.
        // We preserve the same neighbor visitation order: right, down, left, up.
        let mut cnt = 0;
        for i in 0..h {
            for j in 0..w {
                if farm[i][j] == b'+' {
                    continue;
                }
                cnt += 1;
                let moji = farm[i][j];
                // Iterative stack to avoid deep recursion while preserving DFS behavior.
                let mut stack = Vec::new();
                stack.push((i, j));
                while let Some((ci, cj)) = stack.pop() {
                    // bounds checks are guaranteed by how we push neighbors, but check anyway.
                    if ci >= h || cj >= w {
                        continue;
                    }
                    if farm[ci][cj] != moji {
                        continue;
                    }
                    farm[ci][cj] = b'+';
                    // Push neighbors in the same order as the C recursion:
                    // right, down, left, up.
                    if cj + 1 < w {
                        stack.push((ci, cj + 1));
                    }
                    if ci + 1 < h {
                        stack.push((ci + 1, cj));
                    }
                    if cj >= 1 {
                        stack.push((ci, cj - 1));
                    }
                    if ci >= 1 {
                        stack.push((ci - 1, cj));
                    }
                }
            }
        }

        // Match printf("%d\n", cnt);
        println!("{}", cnt);
    }
}
