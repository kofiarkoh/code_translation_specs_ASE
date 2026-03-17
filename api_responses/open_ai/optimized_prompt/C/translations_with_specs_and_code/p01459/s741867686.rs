// AOJ 2342 Light Road
// Translated from C to Rust

use std::io::{self, BufRead};

const QMAX: usize = 51000;
const H_MAX: usize = 103;
const W_MAX: usize = 103;

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
    let mut params = first_line.split_whitespace();
    let h: usize = params.next().unwrap().parse().unwrap();
    let w: usize = params.next().unwrap().parse().unwrap();
    let a_max: usize = params.next().unwrap().parse().unwrap();
    let b_max = a_max;

    let mut map = vec![String::new(); h];
    let mut sr = 0; // start row
    let mut sc = 0; // start column
    let mut gr = 0; // goal row
    let mut gc = 0; // goal column

    for r in 0..h {
        let line = lines.next().unwrap().unwrap();
        map[r] = line.clone();
        for c in 0..w {
            if line.chars().nth(c).unwrap() == 'S' {
                sr = r;
                sc = c;
            }
            if line.chars().nth(c).unwrap() == 'G' {
                gr = r;
                gc = c;
            }
        }
    }

    // Check precondition for start position
    if sr == h - 1 || map[sr + 1].chars().nth(sc).unwrap() == '#' {
        println!("-1");
        return;
    }

    let mut que: Vec<Q> = Vec::with_capacity(QMAX);
    que.push(Q { r: sr + 1, c: sc, d: 2, a: 0, b: 0 });

    let mut mk = vec![vec![vec![vec![vec![0; 11]; 11]; 4]; H_MAX]; W_MAX]; H_MAX];
    mk[sr][sc][0][0][0] = 1;

    let mut ans = 100;
    let mut top = 0;
    let mut end = 1;

    let mv: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let ma: [[isize; 4]; 4] = [[0, -1, -1, 1], [-1, 0, 1, -1], [-1, 1, 0, -1], [1, -1, -1, 0]];
    let mb: [[isize; 4]; 4] = [[0, 1, -1, -1], [1, 0, -1, -1], [-1, -1, 0, 1], [-1, -1, 1, 0]];

    while top != end {
        let current = &que[top];
        let r = current.r;
        let c = current.c;
        let d = current.d;
        let a = current.a;
        let b = current.b;

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
            if nr >= h || nc >= w || map[nr].chars().nth(nc).unwrap() == '#' {
                continue;
            }

            let mut next_a = a;
            let mut next_b = b;

            if na > 0 {
                if a + 1 > a_max || map[r].chars().nth(c).unwrap() == 'S' {
                    continue;
                }
                next_a += 1;
            } else if nb > 0 {
                if b + 1 > b_max || map[r].chars().nth(c).unwrap() == 'S' {
                    continue;
                }
                next_b += 1;
            }

            if mk[nr][nc][nd][next_a][next_b] != 0 {
                continue;
            }

            que.push(Q { r: nr, c: nc, d: nd, a: next_a, b: next_b });
            end = (end + 1) % QMAX;
        }
    }

    if ans > 20 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
