use std::io::{self, Read};

const MAX: usize = 602;

#[derive(Clone, Copy)]
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

impl Default for T {
    fn default() -> Self {
        T {
            used: 0,
            m: 0,
            mx: 0,
            xl: 0,
            xr: 0,
            par: 0,
            ch: 0,
            sib: 0,
        }
    }
}

/* UNION-FIND */
fn find_set(p: &mut [i32], x: i32) -> i32 {
    let px = p[x as usize];
    if x != px {
        let root = find_set(p, px);
        p[x as usize] = root;
    }
    p[x as usize]
}

fn link(p: &mut [i32], rank: &mut [i32], x: i32, y: i32) {
    let (xu, yu) = (x as usize, y as usize);
    if rank[xu] > rank[yu] {
        p[yu] = x;
    } else {
        p[xu] = y;
        if rank[xu] == rank[yu] {
            rank[yu] = rank[yu] + 1;
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
    rel: &Vec<Vec<i32>>,
    tr: &Vec<i32>,
    tbl: &mut [T],
) {
    let r_us = r as usize;
    let c_us = c as usize;
    let this_us = this as usize;

    if r == h - 1 {
        if map[r_us][c_us] == b'.' {
            return;
        }
        if tbl[this_us].xl < 0 {
            tbl[this_us].xl = c;
        } else if c < tbl[this_us].xl {
            tbl[this_us].xl = c;
        }
        if c > tbl[this_us].xr {
            tbl[this_us].xr = c;
        }
        return;
    }
    if map[(r + 1) as usize][c_us] == b'.' {
        return;
    }

    let k = rel[(r + 1) as usize][c_us] as usize;
    let pnode = tr[k];
    if pnode == this {
        return;
    }

    if tbl[this_us].xl < 0 {
        tbl[this_us].xl = c;
    } else if c < tbl[this_us].xl {
        tbl[this_us].xl = c;
    }
    if c > tbl[this_us].xr {
        tbl[this_us].xr = c;
    }
    if tbl[this_us].par != 0 {
        return;
    }

    let p_us = pnode as usize;
    let mut s = tbl[p_us].ch;
    if s == 0 {
        tbl[p_us].ch = this;
    } else {
        while tbl[s as usize].sib != 0 {
            s = tbl[s as usize].sib;
        }
        tbl[s as usize].sib = this;
    }
    tbl[this_us].par = pnode;
}

fn check(this: i32, tbl: &mut [T]) -> bool {
    let this_us = this as usize;

    if tbl[this_us].used != 0 {
        return true;
    }

    let ch = tbl[this_us].ch;
    if ch != 0 && !check(ch, tbl) {
        return false;
    }

    let mut sib = tbl[this_us].sib;
    while sib != 0 {
        if !check(sib, tbl) {
            return false;
        }
        sib = tbl[sib as usize].sib;
    }

    // Match C's double computation: x = 0.5 + (double)mx / m
    // Note: m should be > 0 for any real node; we keep behavior as-is.
    let x = 0.5f64 + (tbl[this_us].mx as f64) / (tbl[this_us].m as f64);
    if x <= (tbl[this_us].xl as f64) || ((tbl[this_us].xr + 1) as f64) <= x {
        return false;
    }

    let par = tbl[this_us].par as usize;
    tbl[par].m += tbl[this_us].m;
    tbl[par].mx += tbl[this_us].mx;
    tbl[this_us].used = 1;
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mv: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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

        let wu = w as usize;
        let hu = h as usize;

        let mut p = vec![0i32; MAX];
        let mut rank = vec![0i32; MAX];

        let mut map: Vec<Vec<u8>> = vec![vec![b'.'; wu]; hu];
        let mut rel: Vec<Vec<i32>> = vec![vec![0i32; wu]; hu];

        // Read map and initialize p/rel like C: i increments over all cells.
        let mut idx: i32 = 0;
        for r in 0..hu {
            let s = it.next().unwrap_or("");
            let bytes = s.as_bytes();
            for c in 0..wu {
                map[r][c] = if c < bytes.len() { bytes[c] } else { b'\0' };
                p[idx as usize] = idx;
                rel[r][c] = idx;
                idx += 1;
            }
        }

        // Union adjacent equal non-dot cells.
        for r in 0..hu {
            for c in 0..wu {
                if map[r][c] == b'.' {
                    continue;
                }
                for &(dr, dc) in &mv {
                    let r2 = r as i32 + dr;
                    let c2 = c as i32 + dc;
                    if 0 <= r2 && r2 < h && 0 <= c2 && c2 < w {
                        let (r2u, c2u) = (r2 as usize, c2 as usize);
                        if map[r2u][c2u] == map[r][c] {
                            let a = rel[r][c];
                            let b = rel[r2u][c2u];
                            union_set(&mut p, &mut rank, a, b);
                        }
                    }
                }
            }
        }

        // Build component table and tree.
        let mut tbl = vec![T::default(); 100];
        let mut tr = vec![-1i32; MAX];
        let mut sz: i32 = 1;

        for r in (0..hu).rev() {
            for c in 0..wu {
                if map[r][c] == b'.' {
                    continue;
                }
                let k = find_set(&mut p, rel[r][c]);
                rel[r][c] = k;
                let ku = k as usize;

                let node_i: i32;
                if tr[ku] < 0 {
                    node_i = sz;
                    tr[ku] = node_i;
                    sz += 1;
                    tbl[node_i as usize].xl = -1; // as in C
                } else {
                    node_i = tr[ku];
                }

                let tp = &mut tbl[node_i as usize];
                tp.m += 1;
                tp.mx += c as i32;

                mktree(
                    r as i32,
                    c as i32,
                    node_i,
                    w,
                    h,
                    &map,
                    &rel,
                    &tr,
                    &mut tbl,
                );
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
