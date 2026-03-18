use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let w: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // Enforce max size per spec (and matches C fixed arrays)
    if h > 512 || w > 512 {
        std::process::exit(1);
    }

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = match it.next() {
            Some(s) => s.as_bytes().to_vec(),
            None => std::process::exit(1),
        };
        // Spec requires each row length matches width; C code does not check W explicitly.
        // Follow spec: error if mismatch.
        if row.len() != w {
            std::process::exit(1);
        }
        grid.push(row);
    }

    // group ids: 0 means unassigned
    let mut group = vec![vec![0usize; w]; h];
    let mut num_groups: usize = 0;

    // Iterative DFS to avoid recursion depth issues on 512x512.
    let dirs: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for y in 0..h {
        for x in 0..w {
            if group[y][x] != 0 {
                continue;
            }
            num_groups += 1;
            let gid = num_groups;

            let mut stack: Vec<(usize, usize)> = Vec::new();
            stack.push((y, x));

            while let Some((cy, cx)) = stack.pop() {
                if group[cy][cx] == gid {
                    continue;
                }
                group[cy][cx] = gid;

                for (dy, dx) in dirs {
                    let ny = cy as isize + dy;
                    let nx = cx as isize + dx;
                    if ny < 0 || nx < 0 || ny >= h as isize || nx >= w as isize {
                        continue;
                    }
                    let nyu = ny as usize;
                    let nxu = nx as usize;
                    if grid[cy][cx] != grid[nyu][nxu] {
                        if group[nyu][nxu] != gid {
                            stack.push((nyu, nxu));
                        }
                    }
                }
            }
        }
    }

    // count[gid][0/1] where index is (cell == '.') as in C: count[gid][S[i][j] == '.']++
    let mut count = vec![[0u64; 2]; num_groups + 1];
    for y in 0..h {
        for x in 0..w {
            let gid = group[y][x];
            let idx = if grid[y][x] == b'.' { 1 } else { 0 };
            count[gid][idx] += 1;
        }
    }

    let mut ans: u64 = 0;
    for gid in 1..=num_groups {
        ans += count[gid][0] * count[gid][1];
    }

    print!("{ans}\n");
}
