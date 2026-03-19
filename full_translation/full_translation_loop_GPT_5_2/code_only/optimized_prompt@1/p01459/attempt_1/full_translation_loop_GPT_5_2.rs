use std::io::{self, Read};

const QMAX: usize = 51000;

#[derive(Copy, Clone, Default)]
struct Q {
    r: u8,
    c: u8,
    d: u8,
    a: u8,
    b: u8,
}

fn main() {
    // Read entire stdin (C uses fgets line-by-line, but semantics here only depend on tokens/lines).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();
    let a_max: usize = it.next().unwrap().parse().unwrap();
    let b_max: usize = a_max;

    // map[103][103] in C; we only need HxW.
    let mut map = vec![vec![b' '; w]; h];
    let (mut sr, mut sc, mut gr, mut gc) = (0usize, 0usize, 0usize, 0usize);

    for r in 0..h {
        let row = it.next().unwrap_or("");
        let bytes = row.as_bytes();
        for c in 0..w {
            let ch = bytes[c];
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

    // Early exit condition exactly as in C.
    if sr == h - 1 || map[sr + 1][sc] == b'#' {
        print!("-1\n");
        return;
    }

    // mk[103][103][4][11][11] in C; A,B are assumed <= 10 by the original.
    // To preserve behavior for any input, we allocate based on given A (and B=A).
    // This matches the C intent without relying on its fixed-size UB when A>10.
    let mut mk = vec![vec![vec![vec![vec![false; b_max + 1]; a_max + 1]; 4]; w]; h];

    let mv: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    let ma: [[i32; 4]; 4] = [
        [0, -1, -1, 1],
        [-1, 0, 1, -1],
        [-1, 1, 0, -1],
        [1, -1, -1, 0],
    ];
    let mb: [[i32; 4]; 4] = [
        [0, 1, -1, -1],
        [1, 0, -1, -1],
        [-1, -1, 0, 1],
        [-1, -1, 1, 0],
    ];

    let mut que = vec![Q::default(); QMAX + 2];
    let mut top: usize = 0;
    let mut end: usize = 1;

    // Initial queue state: start from cell below S, direction=2, a=b=0.
    que[0] = Q {
        r: (sr + 1) as u8,
        c: sc as u8,
        d: 2,
        a: 0,
        b: 0,
    };

    // Mark mk[sr][sc][0][a][b] = 1 for all a,b.
    for a in 0..=a_max {
        for b in 0..=b_max {
            mk[sr][sc][0][a][b] = true;
        }
    }

    let mut ans: i32 = 100;

    while top != end {
        let q = que[top];
        top += 1;
        if top == QMAX {
            top = 0;
        }

        let r = q.r as usize;
        let c = q.c as usize;
        let d = q.d as usize;
        let a = q.a as usize;
        let b = q.b as usize;

        if r == gr && c == gc {
            let s = (a + b) as i32;
            if s < ans {
                ans = s;
            }
            continue;
        }

        if mk[r][c][d][a][b] {
            continue;
        }
        mk[r][c][d][a][b] = true;

        for nd in 0..4usize {
            let nr_i = r as i32 + mv[nd][0];
            let nc_i = c as i32 + mv[nd][1];
            let na = ma[d][nd];
            let nb = mb[d][nd];

            if na < 0 && nb < 0 {
                continue;
            }
            if nr_i < 0 || nr_i >= h as i32 || nc_i < 0 || nc_i >= w as i32 {
                continue;
            }
            let nr = nr_i as usize;
            let nc = nc_i as usize;
            if map[nr][nc] == b'#' {
                continue;
            }

            // Prepare queue entry (may be discarded by later continues, matching C behavior).
            que[end].r = nr as u8;
            que[end].c = nc as u8;
            que[end].d = nd as u8;

            if na > 0 {
                if a + 1 > a_max || map[r][c] == b'S' {
                    continue;
                }
                if mk[nr][nc][nd][a + 1][b] {
                    continue;
                }
                que[end].a = (a + 1) as u8;
                que[end].b = b as u8;
            } else if nb > 0 {
                if b + 1 > b_max || map[r][c] == b'S' {
                    continue;
                }
                if mk[nr][nc][nd][a][b + 1] {
                    continue;
                }
                que[end].a = a as u8;
                que[end].b = (b + 1) as u8;
            } else if na == 0 {
                que[end].a = a as u8;
                que[end].b = b as u8;
                if mk[nr][nc][nd][a][b] {
                    continue;
                }
            }

            end += 1;
            if end == QMAX {
                end = 0;
            }
        }
    }

    if ans > 20 {
        print!("-1\n");
    } else {
        print!("{}\n", ans);
    }
}
