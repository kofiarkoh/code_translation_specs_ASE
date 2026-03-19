use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Fixed internal storage with 1-cell padding on each side.
    // Preconditions state n <= 298 to avoid OOB with this padding.
    const MAXN: usize = 300;
    let mut grid = vec![vec![0u8; MAXN]; MAXN];

    // Read exactly n^2 character cells (token-based, as per split_whitespace).
    // Only 'o' is traversable; everything else is non-traversable.
    for i in 1..=n {
        for j in 1..=n {
            let tok = it.next().unwrap_or("");
            let b = tok.as_bytes().get(0).copied().unwrap_or(b'\0');
            grid[i][j] = b;
        }
    }

    // Fixed traversal storage capacity: at most 100010 cells in any single region.
    const STACK_CAP: usize = 100_010;
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(STACK_CAP);

    let mut regions: i64 = 0;

    for i in 1..=n {
        for j in 1..=n {
            if grid[i][j] == b'o' {
                regions += 1;

                // Mark and flood-fill (4-connected).
                grid[i][j] = 0;
                stack.clear();
                stack.push((i, j));

                while let Some((x, y)) = stack.pop() {
                    // Neighbor checks rely on 1-cell padding to avoid bounds checks.
                    let neighbors = [(x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)];
                    for (nx, ny) in neighbors {
                        if grid[nx][ny] == b'o' {
                            grid[nx][ny] = 0;
                            // If a region exceeds STACK_CAP, preconditions are violated; behavior is undefined.
                            stack.push((nx, ny));
                        }
                    }
                }
            }
        }
    }

    // Output floor((regions - 1) / 3).
    // In Rust, integer division truncates toward zero; for regions >= 0 this matches floor.
    let ans: i64 = (regions - 1) / 3;
    println!("{ans}");
}
