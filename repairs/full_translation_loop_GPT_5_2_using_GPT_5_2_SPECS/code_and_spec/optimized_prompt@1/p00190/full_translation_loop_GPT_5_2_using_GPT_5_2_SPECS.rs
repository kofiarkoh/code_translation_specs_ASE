use std::io::{self, Read};

const HASHSIZ: usize = 900_001;
const MAXQ: usize = 500_000;
const LIM: i32 = 16;

#[derive(Clone, Copy)]
struct HashEntry {
    s: u64,
    m: i8,
}

#[derive(Clone, Copy)]
struct QueEntry {
    s: u64,
    b1: u8,
    b2: u8,
}

const MOVE: [&[i8]; 13] = [
    /*  0 */ &[2],
    /*  1 */ &[2, 5],
    /*  2 */ &[0, 1, 3, 6],
    /*  3 */ &[2, 7],
    /*  4 */ &[5],
    /*  5 */ &[1, 4, 6, 9],
    /*  6 */ &[2, 5, 7, 10],
    /*  7 */ &[3, 6, 8, 11],
    /*  8 */ &[7],
    /*  9 */ &[5, 10],
    /* 10 */ &[6, 9, 11, 12],
    /* 11 */ &[7, 10],
    /* 12 */ &[10],
];

fn lookup(hash: &[HashEntry], s: u64) -> i32 {
    let mut idx = (s as usize) % HASHSIZ;
    loop {
        let e = hash[idx];
        if e.s == 0 {
            return -1;
        }
        if e.s == s {
            return e.m as i32;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }
}

fn insert(hash: &mut [HashEntry], s: u64, m: i8) -> bool {
    let mut idx = (s as usize) % HASHSIZ;
    loop {
        let e = hash[idx];
        if e.s == 0 {
            hash[idx] = HashEntry { s, m };
            return true;
        }
        if e.s == s {
            return false;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }
}

// Move the 4-bit nibble at position p1 into position sp, and clear p1.
// (Caller uses sp as a blank position, so this corresponds to sliding a tile into a blank.)
fn swap_nibble(s: u64, p1: i32, sp: i32) -> u64 {
    let p1_shift = ((12 - p1) << 2) as u32;
    let sp_shift = ((12 - sp) << 2) as u32;
    let n1 = (s >> p1_shift) & 0xF;
    let mut out = s & !(0xFu64 << p1_shift);
    out |= n1 << sp_shift;
    out
}

// BFS with explicit level sentinels, matching the C code's semantics.
// IMPORTANT: In the original C, the queue sentinel uses s==0, which is safe because the goal
// and all reachable states never have packed value 0. We preserve that behavior.
fn sach(
    hash: &mut [HashEntry],
    q: &mut [QueEntry],
    s0: u64,
    b1: i32,
    b2: i32,
    mode: i32,
    mut step: i32,
    lim: i32,
) -> i32 {
    let mut top: usize = 0;
    let mut end: usize = 0;

    q[end] = QueEntry {
        s: s0,
        b1: b1 as u8,
        b2: b2 as u8,
    };
    end += 1;
    q[end] = QueEntry { s: 0, b1: 0, b2: 0 }; // sentinel
    end += 1;

    while top < end {
        let cur = q[top];
        top += 1;

        if cur.s == 0 {
            step += 1;
            // In C, this can overflow fixed storage (undefined). Here we stop if it would.
            if end >= q.len() {
                return lim + 1;
            }
            q[end] = QueEntry { s: 0, b1: 0, b2: 0 };
            end += 1;
            continue;
        }

        if step > lim {
            break;
        }

        let b0 = cur.b1 as i32;
        let b1v = cur.b2 as i32;

        // Expand moves for each of the two blanks.
        for which_blank in 0..2 {
            let blank_pos = if which_blank == 0 { b0 } else { b1v };
            for &x0 in MOVE[blank_pos as usize] {
                let x = x0 as i32;
                if x == b0 || x == b1v {
                    continue;
                }

                let s2 = swap_nibble(cur.s, x, blank_pos);

                if mode != 0 {
                    if lookup(hash, s2) >= 0 {
                        return step;
                    }
                } else if !insert(hash, s2, step as i8) {
                    continue;
                }

                if end >= q.len() {
                    return lim + 1;
                }

                // Update blank positions: the moved tile came from x, so x becomes blank.
                let (nb1, nb2) = if which_blank == 0 { (x, b1v) } else { (b0, x) };
                q[end] = QueEntry {
                    s: s2,
                    b1: nb1 as u8,
                    b2: nb2 as u8,
                };
                end += 1;
            }
        }
    }

    step
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut hash = vec![HashEntry { s: 0, m: 0 }; HASHSIZ];
    let mut q = vec![QueEntry { s: 0, b1: 0, b2: 0 }; MAXQ + 5];

    // Precompute all states within 16 moves of the goal.
    let goal: u64 = 0x123456789ab0;
    insert(&mut hash, goal, 0);
    sach(&mut hash, &mut q, goal, 0, 12, 0, 1, LIM);

    let mut out = String::new();

    loop {
        let first = match it.next() {
            Some(v) => v.parse::<i32>().unwrap(),
            None => break,
        };
        if first < 0 {
            break;
        }

        let mut blank1: i32 = -1;
        let mut blank2: i32 = -1;
        let mut s: u64 = 0;

        let mut p = first;
        for i in 0..13 {
            if i != 0 {
                p = it.next().unwrap().parse::<i32>().unwrap();
            }
            if p == 0 {
                if blank1 < 0 {
                    blank1 = i as i32;
                } else {
                    blank2 = i as i32;
                }
            }
            s = (s << 4) | (p as u64);
        }

        let mut ans = lookup(&hash, s);
        if ans < 0 {
            // Search up to 4 moves from the query side to intersect the precomputed DB.
            let t = sach(&mut hash, &mut q, s, blank1, blank2, 1, 1, 4);
            if t > 4 {
                ans = -1;
            } else {
                ans = t + LIM;
            }
        }

        if ans < 0 {
            out.push_str("NA\n");
        } else {
            out.push_str(&format!("{ans}\n"));
        }
    }

    print!("{out}");
}
