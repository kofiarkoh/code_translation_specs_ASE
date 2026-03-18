use std::io::{self, Read};

fn main() {
    // Read all input bytes to handle grids that may not be whitespace-separated.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Parse first integer n (like scanf("%d",&n))
    while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
        idx += 1;
    }
    let mut n: usize = 0;
    while idx < bytes.len() && !bytes[idx].is_ascii_whitespace() {
        let b = bytes[idx];
        if b.is_ascii_digit() {
            n = n * 10 + (b - b'0') as usize;
        }
        idx += 1;
    }

    // Grid with 1-cell border, initialized to '\0' (matches C's zero-initialized array border).
    let mut d = vec![vec!['\0'; n + 2]; n + 2];

    // Read n*n characters using scanf(" %c") semantics: skip whitespace, then take one byte/char.
    for i in 0..n {
        for j in 0..n {
            while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
                idx += 1;
            }
            // Preconditions guarantee valid input; if not, default to '\0' to avoid panic.
            let ch = if idx < bytes.len() {
                let c = bytes[idx] as char;
                idx += 1;
                c
            } else {
                '\0'
            };
            d[i + 1][j + 1] = ch;
        }
    }

    let dx = [1isize, 0, -1, 0];
    let dy = [0isize, 1, 0, -1];

    let mut c: i32 = 0;

    // BFS queue
    let mut qx: Vec<usize> = Vec::with_capacity(n * n);
    let mut qy: Vec<usize> = Vec::with_capacity(n * n);

    for i in 0..n {
        for j in 0..n {
            if d[i + 1][j + 1] == 'x' {
                continue;
            }

            qx.clear();
            qy.clear();
            qx.push(j + 1);
            qy.push(i + 1);
            d[i + 1][j + 1] = 'x';
            c += 1;

            let mut t: usize = 0;
            while t < qx.len() {
                let x = qx[t];
                let y = qy[t];
                t += 1;

                for k in 0..4 {
                    let nx = (x as isize + dx[k]) as usize;
                    let ny = (y as isize + dy[k]) as usize;

                    if d[ny][nx] != 'o' {
                        continue;
                    }
                    d[ny][nx] = 'x';
                    qx.push(nx);
                    qy.push(ny);
                }
            }
        }
    }

    // Same adjustment rule as C: (c-1)/3
    let ans = (c - 1) / 3;
    println!("{ans}");
}
