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
    let mut params = first_line.split_whitespace();
    let h: usize = params.next().unwrap().parse().unwrap();
    let w: usize = params.next().unwrap().parse().unwrap();
    let a_max: usize = params.next().unwrap().parse().unwrap();
    let b_max = a_max;

    let mut map = [[0; W]; H];
    let mut mk = [[[[[0; 12]; 12]; 4]; 4]; H]; // Increased size to 12 to accommodate a_max + 1
    let mv: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let ma: [[isize; 4]; 4] = [[0, -1, -1, 1], [-1, 0, 1, -1], [-1, 1, 0, -1], [1, -1, -1, 0]];
    let mb: [[isize; 4]; 4] = [[0, 1, -1, -1], [1, 0, -1, -1], [-1, -1, 0, 1], [-1, -1, 1, 0]];

    let mut sr = 0;
    let mut sc = 0;
    let mut gr = 0;
    let mut gc = 0;

    for r in 0..h {
        let line = lines.next().unwrap().unwrap();
        for (c, ch) in line.chars().enumerate() {
            map[r][c] = ch as u8;
            if ch == 'S' {
                sr = r;
                sc = c;
            } else if ch == 'G' {
                gr = r;
                gc = c;
            }
        }
    }

    if sr == h - 1 || (sr + 1 < h && map[sr + 1][sc] == b'#') {
        println!("-1");
        return;
    }

    let mut que: [Q; QMAX] = [Q { r: 0, c: 0, d: 0, a: 0, b: 0 }; QMAX];
    que[0] = Q { r: sr + 1, c: sc, d: 2, a: 0, b: 0 };
    mk[sr][sc][0][0][0] = 1; // Mark the starting position as visited

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

            let mut new_a = a;
            let mut new_b = b;

            if na > 0 {
                if a + 1 > a_max || map[r][c] == b'S' {
                    continue;
                }
                new_a += 1;
            } else if nb > 0 {
                if b + 1 > b_max || map[r][c] == b'S' {
                    continue;
                }
                new_b += 1;
            }

            if new_a <= a_max && new_b <= b_max && mk[nr][nc][nd][new_a][new_b] == 0 {
                que[end] = Q { r: nr, c: nc, d: nd, a: new_a, b: new_b };
                end = (end + 1) % QMAX;
            }
        }
    }

    if ans > 20 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
