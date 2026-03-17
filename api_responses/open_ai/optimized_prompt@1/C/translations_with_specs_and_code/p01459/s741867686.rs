// AOJ 2342 Light Road
// 2018.4.8 bal4u

use std::collections::VecDeque;
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

    let mut map = vec![vec![' '; w]; h];
    let mut start = (0, 0);
    let mut goal = (0, 0);

    for r in 0..h {
        let line = lines.next().unwrap().unwrap();
        for c in 0..w {
            map[r][c] = line.chars().nth(c).unwrap();
            if map[r][c] == 'S' {
                start = (r, c);
            }
            if map[r][c] == 'G' {
                goal = (r, c);
            }
        }
    }

    if start.0 == h - 1 || map[start.0 + 1][start.1] == '#' {
        println!("-1");
        return;
    }

    let mut queue: VecDeque<Q> = VecDeque::new();
    queue.push_back(Q {
        r: start.0 + 1,
        c: start.1,
        d: 2,
        a: 0,
        b: 0,
    });

    let mut mk = vec![vec![vec![vec![vec![0; 11]; 11]; 4]; H_MAX]; W_MAX];
    mk[start.0][start.1][0][0][0] = 1;

    let mv: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let ma: [[isize; 4]; 4] = [[0, -1, -1, 1], [-1, 0, 1, -1], [-1, 1, 0, -1], [1, -1, -1, 0]];
    let mb: [[isize; 4]; 4] = [[0, 1, -1, -1], [1, 0, -1, -1], [-1, -1, 0, 1], [-1, -1, 1, 0]];

    let mut ans = 100;
    while let Some(q) = queue.pop_front() {
        let (r, c, d, a, b) = (q.r, q.c, q.d, q.a, q.b);

        if r == goal.0 && c == goal.1 {
            ans = ans.min(a + b);
            continue;
        }

        if mk[r][c][d][a][b] != 0 {
            continue;
        }
        mk[r][c][d][a][b] = 1;

        for nd in 0..4 {
            let (nr, nc) = (r as isize + mv[nd].0, c as isize + mv[nd].1);
            let (na, nb) = (ma[d][nd], mb[d][nd]);

            if na < 0 && nb < 0 {
                continue;
            }
            if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize || map[nr as usize][nc as usize] == '#' {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            let (new_a, new_b) = if na > 0 {
                if a + 1 > a_max || map[r][c] == 'S' {
                    continue;
                }
                (a + 1, b)
            } else if nb > 0 {
                if b + 1 > b_max || map[r][c] == 'S' {
                    continue;
                }
                (a, b + 1)
            } else {
                (a, b)
            };

            if mk[nr][nc][nd][new_a][new_b] == 0 {
                queue.push_back(Q {
                    r: nr,
                    c: nc,
                    d: nd,
                    a: new_a,
                    b: new_b,
                });
            }
        }
    }

    if ans > 20 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
