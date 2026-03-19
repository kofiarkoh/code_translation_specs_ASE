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

fn find_set(p: &mut [i32], mut x: i32) -> i32 {
    // Iterative path compression to avoid recursion overhead.
    let mut root = x;
    while p[root as usize] != root {
        root = p[root as usize];
    }
    while p[x as usize] != x {
        let parent = p[x as usize];
        p[x as usize] = root;
        x = parent;
    }
    root
}

fn link(p: &mut [i32], rank: &mut [i32], x: i32, y: i32) {
    let xi = x as usize;
    let yi = y as usize;
    if rank[xi] > rank[yi] {
        p[yi] = x;
    } else {
        p[xi] = y;
        if rank[xi] == rank[yi] {
            rank[yi] += 1;
        }
    }
}

fn union_set(p: &mut [i32], rank: &mut [i32], x: i32, y: i32) {
    let rx = find_set(p, x);
    let ry = find_set(p, y);
    if rx != ry {
        link(p, rank, rx, ry);
    }
}

fn mktree(
    r: usize,
    c: usize,
    this: i32,
    h: usize,
    map: &[Vec<u8>],
    rel: &[Vec<i32>],
    tbl: &mut [T],
    tr: &[i32],
) {
    let thisu = this as usize;

    if r == h - 1 {
        if map[r][c] == b'.' {
            return;
        }
        if tbl[thisu].xl < 0 {
            tbl[thisu].xl = c as i32;
        } else if (c as i32) < tbl[thisu].xl {
            tbl[thisu].xl = c as i32;
        }
        if (c as i32) > tbl[thisu].xr {
            tbl[thisu].xr = c as i32;
        }
        return;
    }
    if map[r + 1][c] == b'.' {
        return;
    }

    let k = rel[r + 1][c] as usize;
    let pnode = tr[k];
    if pnode == this {
        return;
    }

    if tbl[thisu].xl < 0 {
        tbl[thisu].xl = c as i32;
    } else if (c as i32) < tbl[thisu].xl {
        tbl[thisu].xl = c as i32;
    }
    if (c as i32) > tbl[thisu].xr {
        tbl[thisu].xr = c as i32;
    }
    if tbl[thisu].par != 0 {
        return;
    }

    // Attach `this` as the last child of `pnode`.
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
}

fn check(root: i32, tbl: &mut [T]) -> bool {
    // Post-order traversal without recursion to avoid potential deep recursion.
    // We must emulate the C order: check(child) then all siblings, then process node.
    let mut stack: Vec<(i32, u8)> = Vec::new(); // (node, state 0=enter,1=exit)
    stack.push((root, 0));

    while let Some((v, state)) = stack.pop() {
        let vu = v as usize;
        if state == 0 {
            if tbl[vu].used != 0 {
                continue;
            }
            // Exit state after children processed.
            stack.push((v, 1));

            // Push siblings chain and child in the same order as recursive:
            // check(ch) first, then check(sib) in order.
            // With a stack (LIFO), push in reverse: siblings (reverse order), then child.
            let ch = tbl[vu].ch;
            if ch != 0 {
                // Collect siblings starting from ch, then push reversed.
                let mut nodes: Vec<i32> = Vec::new();
                let mut cur = ch;
                while cur != 0 {
                    nodes.push(cur);
                    cur = tbl[cur as usize].sib;
                }
                for &node in nodes.iter().rev() {
                    if tbl[node as usize].used == 0 {
                        stack.push((node, 0));
                    }
                }
            }
        } else {
            if tbl[vu].used != 0 {
                continue;
            }
            // All descendants should have been processed; if any was unstable, we'd have returned.
            let x = 0.5f64 + (tbl[vu].mx as f64) / (tbl[vu].m as f64);
            if x <= (tbl[vu].xl as f64) || ((tbl[vu].xr + 1) as f64) <= x {
                return false;
            }
            let par = tbl[vu].par as usize;
            tbl[par].m += tbl[vu].m;
            tbl[par].mx += tbl[vu].mx;
            tbl[vu].used = 1;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let w = match it.next() {
            None => break,
            Some(s) => s.parse::<usize>().unwrap(),
        };
        let h = it.next().unwrap().parse::<usize>().unwrap();
        if w == 0 {
            break;
        }

        // Arrays sized like the C code (fixed MAX), but we only use w*h entries.
        let mut p = vec![0i32; MAX];
        let mut rank = vec![0i32; MAX];

        let mut map: Vec<Vec<u8>> = vec![vec![b'.'; w]; h];
        let mut rel: Vec<Vec<i32>> = vec![vec![0i32; w]; h];

        let mut idx: i32 = 0;
        for r in 0..h {
            let s = it.next().unwrap();
            let bytes = s.as_bytes();
            for c in 0..w {
                map[r][c] = bytes[c];
                p[idx as usize] = idx;
                rel[r][c] = idx;
                idx += 1;
            }
        }

        let mv = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
        for r in 0..h {
            for c in 0..w {
                if map[r][c] == b'.' {
                    continue;
                }
                for (dr, dc) in mv {
                    let r2 = r as i32 + dr;
                    let c2 = c as i32 + dc;
                    if r2 >= 0 && r2 < h as i32 && c2 >= 0 && c2 < w as i32 {
                        let r2u = r2 as usize;
                        let c2u = c2 as usize;
                        if map[r2u][c2u] == map[r][c] {
                            let a = rel[r][c];
                            let b = rel[r2u][c2u];
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
                if map[r][c] == b'.' {
                    continue;
                }
                let k = find_set(&mut p, rel[r][c]);
                rel[r][c] = k;
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
                tbl[i as usize].mx += c as i32;

                mktree(r, c, i, h, &map, &rel, &mut tbl, &tr);
            }
        }
        tbl[1].par = 1;

        out.push_str(if check(1, &mut tbl) { "STABLE\n" } else { "UNSTABLE\n" });
    }

    print!("{out}");
}
