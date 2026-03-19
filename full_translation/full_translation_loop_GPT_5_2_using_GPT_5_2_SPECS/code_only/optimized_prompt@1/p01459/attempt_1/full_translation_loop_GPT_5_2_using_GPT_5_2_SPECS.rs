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
    // Read entire stdin; C uses fgets/sscanf and line reads, but semantics here
    // only depend on tokens for H,W,A and then H grid lines.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let w: i32 = it.next().unwrap().parse().unwrap();
    let a_max: i32 = it.next().unwrap().parse().unwrap();
    let b_max: i32 = a_max;

    // map[103][103] in C; we only use [0..H)[0..W)
    let mut map = vec![vec![b'\n'; 103]; 103];

    let (mut sr, mut sc, mut gr, mut gc) = (0i32, 0i32, 0i32, 0i32);

    for r in 0..h {
        let line = it.next().unwrap_or("");
        let bytes = line.as_bytes();
        for c in 0..w {
            let ch = if (c as usize) < bytes.len() { bytes[c as usize] } else { b'\n' };
            map[r as usize][c as usize] = ch;
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

    // If start is on bottom row or cell below is wall => -1
    if sr == h - 1 || map[(sr + 1) as usize][sc as usize] == b'#' {
        print!("-1\n");
        return;
    }

    // mk[103][103][4][11][11] in C; A,B are assumed <= 10 by the original.
    // We allocate fixed sizes to match C indexing behavior.
    let mut mk = vec![vec![vec![vec![vec![false; 11]; 11]; 4]; 103]; 103];

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

    // Initial queue state: move one step down from S, direction=2, a=b=0
    que[0] = Q {
        r: (sr + 1) as u8,
        c: sc as u8,
        d: 2,
        a: 0,
        b: 0,
    };

    // Mark all mk[sr][sc][0][a][b] = 1 for a,b in [0..=A]
    // (This is exactly what the C code does, even though d=0 is arbitrary here.)
    for aa in 0..=a_max {
        for bb in 0..=b_max {
            if aa >= 0 && aa <= 10 && bb >= 0 && bb <= 10 {
                mk[sr as usize][sc as usize][0][aa as usize][bb as usize] = true;
            }
        }
    }

    let mut ans: i32 = 100;

    while top != end {
        let q = que[top];
        let r = q.r as i32;
        let c = q.c as i32;
        let d = q.d as usize;
        let a = q.a as i32;
        let b = q.b as i32;

        top += 1;
        if top == QMAX {
            top = 0;
        }

        if r == gr && c == gc {
            let s = a + b;
            if s < ans {
                ans = s;
            }
            continue;
        }

        // If already visited, skip
        if a >= 0 && a <= 10 && b >= 0 && b <= 10 {
            if mk[r as usize][c as usize][d][a as usize][b as usize] {
                continue;
            }
            mk[r as usize][c as usize][d][a as usize][b as usize] = true;
        } else {
            // Out-of-range a/b would be UB in C; original constraints avoid it.
            continue;
        }

        for nd in 0..4usize {
            let nr = r + mv[nd][0];
            let nc = c + mv[nd][1];
            let na = ma[d][nd];
            let nb = mb[d][nd];

            if na < 0 && nb < 0 {
                continue;
            }
            if nr < 0 || nr >= h || nc < 0 || nc >= w {
                continue;
            }
            if map[nr as usize][nc as usize] == b'#' {
                continue;
            }

            // Prepare queue entry; may be discarded by later continues (matching C).
            que[end].r = nr as u8;
            que[end].c = nc as u8;
            que[end].d = nd as u8;

            if na > 0 {
                if a + 1 > a_max || map[r as usize][c as usize] == b'S' {
                    continue;
                }
                if mk[nr as usize][nc as usize][nd][(a + 1) as usize][b as usize] {
                    continue;
                }
                que[end].a = (a + 1) as u8;
                que[end].b = b as u8;
            } else if nb > 0 {
                if b + 1 > b_max || map[r as usize][c as usize] == b'S' {
                    continue;
                }
                if mk[nr as usize][nc as usize][nd][a as usize][(b + 1) as usize] {
                    continue;
                }
                que[end].a = a as u8;
                que[end].b = (b + 1) as u8;
            } else if na == 0 {
                que[end].a = a as u8;
                que[end].b = b as u8;
                if mk[nr as usize][nc as usize][nd][a as usize][b as usize] {
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
