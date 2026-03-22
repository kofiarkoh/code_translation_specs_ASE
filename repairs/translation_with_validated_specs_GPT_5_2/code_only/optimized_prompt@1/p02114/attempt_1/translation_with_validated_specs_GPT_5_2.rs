use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse like scanf would (whitespace-separated tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C: char d[300][300] = {}; -> zero-initialized.
    // We'll use u8 and keep the same indexing scheme (1..=n used for input),
    // relying on the zero border to stop BFS (since 0 - b'o' != 0).
    let mut d = vec![vec![0u8; 300]; 300];

    for i in 0..n {
        for j in 0..n {
            let tok = it.next().unwrap_or("");
            let ch = tok.as_bytes().get(0).copied().unwrap_or(0);
            d[i + 1][j + 1] = ch;
        }
    }

    // Direction arrays as in C.
    let xdir: [i32; 4] = [1, 0, -1, 0];
    let ydir: [i32; 4] = [0, 1, 0, -1];

    // C arrays sized 100010; keep same.
    let mut x = vec![0i32; 100_010];
    let mut y = vec![0i32; 100_010];

    let mut c: i32 = 0;

    for i in 0..n {
        for j in 0..n {
            if d[i + 1][j + 1] == b'x' {
                continue;
            }

            // d[y[t=0]=i+1][x[0]=j+1]='x';
            let mut t: usize = 0;
            y[t] = (i + 1) as i32;
            x[0] = (j + 1) as i32;
            d[(i + 1)][(j + 1)] = b'x';

            c += 1;

            // for(r=1; r-t; t++) { ... }
            let mut r: usize = 1;
            while (r as i32 - t as i32) != 0 {
                for k in 0..4 {
                    let ny = y[t] + ydir[k];
                    let nx = x[t] + xdir[k];

                    // In C, ny/nx are always within [0,299] due to the zero border and n<=? implicit.
                    // We mirror that assumption; if violated, C would have UB anyway.
                    let nyu = ny as usize;
                    let nxu = nx as usize;

                    // if(d[ny][nx]-'o')continue;
                    // This is equivalent to: if d[ny][nx] != 'o' continue;
                    if d[nyu][nxu].wrapping_sub(b'o') != 0 {
                        continue;
                    }

                    // d[y[r]=ny][x[r]=nx]='x'; r++;
                    y[r] = ny;
                    x[r] = nx;
                    d[nyu][nxu] = b'x';
                    r += 1;
                }
                t += 1;
            }
        }
    }

    // printf("%d\n",(c-1)/3);
    let ans = (c - 1) / 3;
    print!("{ans}\n");
}
