use std::io::{self, Read};

const MAX: usize = 602;

#[derive(Clone, Copy, Default)]
struct T {
    used: i32,
    m: i32,
    mx: i32,
    xl: i32,
    xr: i32,
    par: i32,
    ch: i32,
    sib: i32,
}

fn find_set(p: &mut [i32], x: i32) -> i32 {
    let xi = x as usize;
    if p[xi] != x {
        let root = find_set(p, p[xi]);
        p[xi] = root;
    }
    p[xi]
}

fn link(p: &mut [i32], rank: &mut [i32], x: i32, y: i32) {
    let xi = x as usize;
    let yi = y as usize;
    if rank[xi] > rank[yi] {
        p[yi] = x;
    } else {
        p[xi] = y;
        if rank[xi] == rank[yi] {
            rank[yi] = rank[yi] + 1;
        }
    }
}

fn union_set(p: &mut [i32], rank: &mut [i32], x: i32, y: i32) {
    let rx = find_set(p, x);
    let ry = find_set(p, y);
    link(p, rank, rx, ry);
}

fn mktree(
    r: i32,
    c: i32,
    this: i32,
    w: i32,
    h: i32,
    map: &[Vec<u8>],
    rel: &[Vec<i32>],
    tbl: &mut [T],
    tr: &[i32],
) {
    let thisu = this as usize;

    if r == h - 1 {
        if map[r as usize][c as usize] == b'.' {
            return;
        }
        if tbl[thisu].xl < 0 {
            tbl[thisu].xl = c;
        } else if c < tbl[thisu].xl {
            tbl[thisu].xl = c;
        }
        if c > tbl[thisu].xr {
            tbl[thisu].xr = c;
        }
        return;
    }
    if map[(r + 1) as usize][c as usize] == b'.' {
        return;
    }

    let k = rel[(r + 1) as usize][c as usize] as usize;
    let pnode = tr[k];
    if pnode == this {
        return;
    }

    if tbl[thisu].xl < 0 {
        tbl[thisu].xl = c;
    } else if c < tbl[thisu].xl {
        tbl[thisu].xl = c;
    }
    if c > tbl[thisu].xr {
        tbl[thisu].xr = c;
    }
    if tbl[thisu].par != 0 {
        return;
    }

    let pidx = pnode as usize;
    let mut s = tbl[pidx].ch;
    if s == 0 {
        tbl[pidx].ch = this;
    } else {
        while tbl[s as usize].sib != 0 {
            s = tbl[s as usize].sib;
        }
        tbl[s as usize].sib = this;
    }
    tbl[thisu].par = pnode;

    let _ = w; // kept to mirror signature/intent; unused like in C.
}

fn check(this: i32, tbl: &mut [T]) -> bool {
    let thisu = this as usize;
    if tbl[thisu].used != 0 {
        return true;
    }

    let ch = tbl[thisu].ch;
    if ch != 0 && !check(ch, tbl) {
        return false;
    }

    let mut sib = tbl[thisu].sib;
    while sib != 0 {
        if !check(sib, tbl) {
            return false;
        }
        sib = tbl[sib as usize].sib;
    }

    // C uses double: x = 0.5 + (double)mx / m;
    // m is positive for any created node; preserve floating behavior.
    let x = 0.5f64 + (tbl[thisu].mx as f64) / (tbl[thisu].m as f64);
    if x <= (tbl[thisu].xl as f64) || ((tbl[thisu].xr + 1) as f64) <= x {
        return false;
    }

    let par = tbl[thisu].par as usize;
    tbl[par].m += tbl[thisu].m;
    tbl[par].mx += tbl[thisu].mx;
    tbl[thisu].used = 1;
    true
}

fn main() {
    // Token-based input to emulate scanf("%d%d") and scanf("%s") behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break;
        }
        let w: i32 = w_opt.unwrap().parse().unwrap();
        let h: i32 = it.next().unwrap().parse().unwrap();
        if w <= 0 {
            break;
        }

        let mut p = vec![0i32; MAX];
        let mut rank = vec![0i32; MAX];

        let mut map: Vec<Vec<u8>> = vec![vec![b'.'; w as usize]; h as usize];
        let mut rel: Vec<Vec<i32>> = vec![vec![0i32; w as usize]; h as usize];

        let mut idx: i32 = 0;
        for r in 0..h as usize {
            let s = it.next().unwrap_or("");
            let bytes = s.as_bytes();
            for c in 0..w as usize {
                map[r][c] = bytes[c];
                p[idx as usize] = idx;
                rel[r][c] = idx;
                idx += 1;
            }
        }

        let mv = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        for r in 0..h {
            for c in 0..w {
                if map[r as usize][c as usize] == b'.' {
                    continue;
                }
                for (dr, dc) in mv {
                    let r2 = r + dr;
                    let c2 = c + dc;
                    if 0 <= r2 && r2 < h && 0 <= c2 && c2 < w {
                        if map[r2 as usize][c2 as usize] == map[r as usize][c as usize] {
                            let a = rel[r as usize][c as usize];
                            let b = rel[r2 as usize][c2 as usize];
                            union_set(&mut p, &mut rank, a, b);
                        }
                    }
                }
            }
        }

        let mut tbl = vec![T::default(); 100];
        let mut tr = vec![-1i32; MAX];

        let mut sz: i32 = 1;
        for r in (0..h).rev() {
            for c in 0..w {
                if map[r as usize][c as usize] == b'.' {
                    continue;
                }
                let k = find_set(&mut p, rel[r as usize][c as usize]);
                rel[r as usize][c as usize] = k;
                let ku = k as usize;

                let i: i32;
                if tr[ku] < 0 {
                    i = sz;
                    tr[ku] = sz;
                    sz += 1;
                    tbl[i as usize].xl = -1;
                } else {
                    i = tr[ku];
                }

                tbl[i as usize].m += 1;
                tbl[i as usize].mx += c;

                mktree(r, c, i, w, h, &map, &rel, &mut tbl, &tr);
            }
        }
        tbl[1].par = 1;

        if check(1, &mut tbl) {
            out.push_str("STABLE\n");
        } else {
            out.push_str("UNSTABLE\n");
        }
    }

    print!("{out}");
}
