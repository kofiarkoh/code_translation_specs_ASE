use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    const MAXN: usize = 300; // includes 1-cell padding; precondition implies n <= 298
    let mut grid = vec![vec![0u8; MAXN]; MAXN];

    // The grid is provided as n tokens, each typically a string of length n (e.g., "oxooo").
    // Read n rows; if a token is shorter than n, missing cells become non-'o'.
    for i in 1..=n {
        let row = it.next().unwrap_or("");
        let bytes = row.as_bytes();
        for j in 1..=n {
            grid[i][j] = *bytes.get(j - 1).unwrap_or(&b'\0');
        }
    }

    const STACK_CAP: usize = 100_010; // precondition: any single region <= this many cells
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(STACK_CAP);

    let mut regions: i64 = 0;

    for i in 1..=n {
        for j in 1..=n {
            if grid[i][j] == b'o' {
                regions += 1;

                grid[i][j] = 0;
                stack.clear();
                stack.push((i, j));

                while let Some((x, y)) = stack.pop() {
                    // Use padding to avoid bounds checks; indices stay within 0..MAXN under preconditions.
                    let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                    for (nx, ny) in neighbors {
                        if grid[nx][ny] == b'o' {
                            grid[nx][ny] = 0;
                            // If this exceeds STACK_CAP, preconditions are violated; behavior is undefined.
                            stack.push((nx, ny));
                        }
                    }
                }
            }
        }
    }

    // Output floor((regions - 1) / 3). For regions >= 0, Rust's truncating division matches floor.
    let ans: i64 = (regions - 1) / 3;
    println!("{ans}");
}
