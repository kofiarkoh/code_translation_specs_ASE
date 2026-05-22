use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-delimited tokens (matches scanf("%d%d") and scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1), // undefined output per spec on read failure
    };
    let w: usize = match it.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // Read H rows as "words". Only first W characters are used.
    let mut s: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = match it.next() {
            Some(r) => r.as_bytes(),
            None => std::process::exit(1),
        };
        // Spec guarantees length at least W; if not, behavior is undefined in C.
        // Here we still index assuming that precondition holds.
        s.push(row[..w].to_vec());
    }

    // group[y][x] == 0 means unassigned; otherwise region id starting at 1.
    let mut group = vec![vec![0usize; w]; h];

    // Iterative DFS to avoid recursion depth issues.
    let dirs: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut regions: usize = 0;
    for y in 0..h {
        for x in 0..w {
            if group[y][x] != 0 {
                continue;
            }
            regions += 1;
            let id = regions;

            let mut stack: Vec<(usize, usize)> = Vec::new();
            stack.push((y, x));
            while let Some((cy, cx)) = stack.pop() {
                if group[cy][cx] == id {
                    continue;
                }
                group[cy][cx] = id;

                for (dy, dx) in dirs {
                    let ny = cy as isize + dy;
                    let nx = cx as isize + dx;
                    if ny < 0 || nx < 0 || ny >= h as isize || nx >= w as isize {
                        continue;
                    }
                    let nyu = ny as usize;
                    let nxu = nx as usize;
                    if s[cy][cx] != s[nyu][nxu] && group[nyu][nxu] != id {
                        stack.push((nyu, nxu));
                    }
                }
            }
        }
    }

    // count[id][0] = non-dot, count[id][1] = dot (matches C's boolean index).
    let mut count = vec![[0u64; 2]; regions + 1];
    for y in 0..h {
        for x in 0..w {
            let id = group[y][x];
            let is_dot = (s[y][x] == b'.') as usize;
            count[id][is_dot] += 1;
        }
    }

    let mut ans: u64 = 0;
    for id in 1..=regions {
        ans = ans.wrapping_add(count[id][0].wrapping_mul(count[id][1]));
    }

    // Print as unsigned 64-bit decimal with newline.
    println!("{}", ans);
}
