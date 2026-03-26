// AOJ 2342 Light Road
// 2018.4.8 bal4u

use std::io::{self, BufRead};

const QMAX: usize = 51000;
const H: usize = 103;
const W: usize = 103;

#[derive(Clone, Copy)]
struct Q {
    r: usize,
    c: usize,
    d: usize,
    a: usize,
    b: usize,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let h: usize = parts.next().unwrap().parse().unwrap();
    let w: usize = parts.next().unwrap().parse().unwrap();
    let a_limit: usize = parts.next().unwrap().parse().unwrap();
    let b_limit = a_limit;

    let mut map = [[0u8; W]; H];
    let mut mk = [[[[[0; 11]; 11]; 4]; H]; W];
    let mv: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let ma: [[isize; 4]; 4] = [[0, -1, -1, 1], [-1, 0, 1, -1], [-1, 1, 0, -1], [1, -1, -1, 0]];
    let mb: [[isize; 4]; 4] = [[0, 1, -1, -1], [1, 0, -1, -1], [-1, -1, 0, 1], [-1, -1, 1, 0]];

    let mut sr = 0;
    let mut sc = 0;
    let mut gr = 0;
    let mut gc = 0;

    for r in 0..h {
        let line = lines.next().unwrap().unwrap();
        for (c, ch) in line.bytes().enumerate() {
            map[r][c] = ch;
            if ch == b'S' {
                sr = r;
                sc = c;
            }
            if ch == b'G' {
                gr = r;
                gc = c;
            }
        }
    }

    if sr == h - 1 || map[sr + 1][sc] == b'#' {
        println!("-1");
        return;
    }

    let mut que = vec![Q { r: 0, c: 0, d: 0, a: 0, b: 0 }; QMAX];
    que[0] = Q { r: sr + 1, c: sc, d: 2, a: 0, b: 0 };
    for a in 0..=a_limit {
        for b in 0..=b_limit {
            mk[sr][sc][0][a][b] = 1;
        }
    }

    let mut top = 0;
    let mut end = 1;
    let mut ans = 100;

    while top != end {
        let Q { r, c, d, a, b } = que[top];
        top = (top + 1) % QMAX;

        if r == gr && c == gc {
            if a + b < ans {
                ans = a + b;
            }
            continue;
        }

        if mk[r][c][d][a][b] != 0 {
            continue;
        }
        mk[r][c][d][a][b] = 1;

        for nd in 0..4 {
            let nr = (r as isize + mv[nd].0) as usize;
            let nc = (c as isize + mv[nd].1) as usize;
            let na = ma[d][nd];
            let nb = mb[d][nd];

            if na < 0 && nb < 0 {
                continue;
            }
            if nr >= h || nc >= w || map[nr][nc] == b'#' {
                continue;
            }

            que[end] = Q { r: nr, c: nc, d: nd, a: a, b: b }; // Preserve a and b initially
            if na > 0 {
                if a + 1 > a_limit || map[r][c] == b'S' {
                    continue;
                }
                if mk[nr][nc][nd][a + 1][b] != 0 {
                    continue;
                }
                que[end].a = a + 1;
            } else if nb > 0 {
                if b + 1 > b_limit || map[r][c] == b'S' {
                    continue;
                }
                if mk[nr][nc][nd][a][b + 1] != 0 {
                    continue;
                }
                que[end].b = b + 1;
            } else if na == 0 {
                if mk[nr][nc][nd][a][b] != 0 {
                    continue;
                }
            }
            end = (end + 1) % QMAX;
        }
    }

    if ans > 20 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
