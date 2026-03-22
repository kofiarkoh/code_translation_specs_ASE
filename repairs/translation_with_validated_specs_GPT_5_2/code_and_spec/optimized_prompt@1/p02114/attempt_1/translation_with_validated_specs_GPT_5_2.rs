use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // C code uses a 1-cell border around the n x n grid (size 300x300, zero-initialized).
    // We'll mirror that with (n+2) x (n+2), initialized to '\0'.
    let mut d = vec![vec!['\0'; n + 2]; n + 2];

    for i in 0..n {
        for j in 0..n {
            let ch = it.next().unwrap().chars().next().unwrap();
            d[i + 1][j + 1] = ch;
        }
    }

    let dx = [1i32, 0, -1, 0];
    let dy = [0i32, 1, 0, -1];

    // BFS queue storage sized per spec (n^2 <= 100010).
    let mut qx = vec![0usize; 100_010];
    let mut qy = vec![0usize; 100_010];

    let mut c: i32 = 0;

    for i in 0..n {
        for j in 0..n {
            // The C code skips any cell already marked 'x' (visited or originally 'x').
            if d[i + 1][j + 1] == 'x' {
                continue;
            }

            // Start a BFS from this cell regardless of whether it's 'o' or not.
            // This matches the C behavior: it increments c for every non-'x' cell,
            // but only expands to neighbors that are exactly 'o'.
            let mut t: usize = 0;
            let mut r: usize = 1;

            qy[0] = i + 1;
            qx[0] = j + 1;
            d[i + 1][j + 1] = 'x';
            c += 1;

            while r > t {
                let cy = qy[t] as i32;
                let cx = qx[t] as i32;
                t += 1;

                for k in 0..4 {
                    let ny = (cy + dy[k]) as usize;
                    let nx = (cx + dx[k]) as usize;
                    if d[ny][nx] != 'o' {
                        continue;
                    }
                    d[ny][nx] = 'x';
                    qy[r] = ny;
                    qx[r] = nx;
                    r += 1;
                }
            }
        }
    }

    // Output floor((C-1)/3) with C as computed above.
    // Note: This follows the C code exactly. The spec defines C as the number of
    // connected components of 'o' cells; the C code actually counts all non-'x'
    // cells as components (with 'o' regions merged). If the input contains only
    // 'o' and 'x', these coincide with the spec's intent.
    let ans = (c - 1) / 3;
    print!("{ans}\n");
}
