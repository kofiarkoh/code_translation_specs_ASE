use std::io::{self, Read};

const MAX: usize = 500_000;
const HASHSIZ: usize = 900_001;

#[derive(Clone, Copy)]
struct HashEnt {
    s: u64,
    m: i8,
}

#[derive(Clone, Copy)]
struct QueEnt {
    s: u64,
    b1: i8,
    b2: i8,
}

const MOVE: [[i8; 5]; 13] = [
    /*  0 */ [2, -1, -1, -1, -1],
    /*  1 */ [2, 5, -1, -1, -1],
    /*  2 */ [0, 1, 3, 6, -1],
    /*  3 */ [2, 7, -1, -1, -1],
    /*  4 */ [5, -1, -1, -1, -1],
    /*  5 */ [1, 4, 6, 9, -1],
    /*  6 */ [2, 5, 7, 10, -1],
    /*  7 */ [3, 6, 8, 11, -1],
    /*  8 */ [7, -1, -1, -1, -1],
    /*  9 */ [5, 10, -1, -1, -1],
    /* 10 */ [6, 9, 11, 12, -1],
    /* 11 */ [7, 10, -1, -1, -1],
    /* 12 */ [10, -1, -1, -1, -1],
];

fn lookup(hash: &[HashEnt], s: u64) -> i8 {
    let mut idx = (s as usize) % HASHSIZ;
    loop {
        let e = hash[idx];
        if e.s == 0 {
            return -1;
        }
        if e.s == s {
            return e.m;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }
}

fn insert(hash: &mut [HashEnt], s: u64, m: i8) -> bool {
    let mut idx = (s as usize) % HASHSIZ;
    loop {
        let e = hash[idx];
        if e.s == 0 {
            hash[idx] = HashEnt { s, m };
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

// Matches the C behavior: move the nibble at p1 into sp, and clear p1.
// (The caller ensures sp is a blank position.)
fn swap_nibble(mut s: u64, p1: i8, sp: i8) -> u64 {
    let p1s = ((12 - p1 as i32) << 2) as u32;
    let sps = ((12 - sp as i32) << 2) as u32;
    let n1 = (s >> p1s) & 0xF;
    s &= !(0xFULL << p1s);
    s |= n1 << sps;
    s
}

// Bidirectional-ish search helper as in C:
// mode=0: build hash table from goal out to lim (16) storing distance
// mode=1: search from input up to lim (4) and if hits hash, return step
fn sach(
    hash: &mut [HashEnt],
    q: &mut [QueEnt],
    s0: u64,
    b1: i8,
    b2: i8,
    mode: i32,
    mut step: i8,
    lim: i8,
) -> i8 {
    let mut top: usize = 0;
    let mut end: usize = 0;

    q[end] = QueEnt { s: s0, b1, b2 };
    end += 1;
    q[end] = QueEnt { s: 0, b1: 0, b2: 0 }; // sentinel
    end += 1;

    while top < end {
        let cur = q[top];
        top += 1;

        if cur.s == 0 {
            step = step.wrapping_add(1);
            // push next level sentinel
            q[end] = QueEnt { s: 0, b1: 0, b2: 0 };
            end += 1;
            continue;
        }
        if step > lim {
            break;
        }

        let blanks = [cur.b1, cur.b2];
        for i in 0..2 {
            let bi = blanks[i];
            for &x in MOVE[bi as usize].iter() {
                if x < 0 {
                    break;
                }
                if x == blanks[0] || x == blanks[1] {
                    continue;
                }
                let s2 = swap_nibble(cur.s, x, bi);

                if mode != 0 {
                    if lookup(hash, s2) >= 0 {
                        return step;
                    }
                } else {
                    if !insert(hash, s2, step) {
                        continue;
                    }
                }

                // enqueue
                if end >= q.len() {
                    // Should not happen under stated constraints; keep semantics safe.
                    // If it does, stop expanding further.
                    return step;
                }
                if i == 1 {
                    q[end] = QueEnt {
                        s: s2,
                        b1: blanks[0],
                        b2: x,
                    };
                } else {
                    q[end] = QueEnt {
                        s: s2,
                        b1: x,
                        b2: blanks[1],
                    };
                }
                end += 1;
            }
        }
    }
    step
}

fn main() {
    // Read all ints
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut hash = vec![
        HashEnt { s: 0, m: 0 };
        HASHSIZ + 5
    ];
    let mut q = vec![
        QueEnt { s: 0, b1: 0, b2: 0 };
        MAX + 5
    ];

    let lim: i8 = 16;

    // Precompute from goal state
    let goal: u64 = 0x123456789ab0u64;
    insert(&mut hash, goal, 0);
    sach(&mut hash, &mut q, goal, 0, 12, 0, 1, lim);

    let mut out = String::new();

    loop {
        // Need exactly 13 integers per case; terminate if first is negative.
        let p0 = match it.next() {
            None => break,
            Some(tok) => tok.parse::<i32>().unwrap(),
        };
        if p0 < 0 {
            break;
        }

        let mut s: u64 = 0;
        let mut b1: i8 = -1;
        let mut b2: i8 = -1;

        // i=0
        let mut p = p0;
        if p == 0 {
            b1 = 0;
        }
        s = (s << 4) | (p as u64);

        for i in 1..13 {
            p = it.next().unwrap().parse::<i32>().unwrap();
            if p == 0 {
                if b1 < 0 {
                    b1 = i as i8;
                } else {
                    b2 = i as i8;
                }
            }
            s = (s << 4) | (p as u64);
        }

        let mut ans = lookup(&hash, s) as i32;
        if ans < 0 {
            let t = sach(&mut hash, &mut q, s, b1, b2, 1, 1, 4) as i32;
            if t > 4 {
                ans = -1;
            } else {
                ans = t + (lim as i32);
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
