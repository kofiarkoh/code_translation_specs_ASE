use std::io::{self, Read};

const MAX: usize = 500000;
const HASHSIZ: usize = 900001;

#[derive(Clone, Copy)]
struct HashEnt {
    s: u64,
    m: i8, // stores step (0..16) or -1 not used; C uses char
}

#[derive(Clone, Copy)]
struct QueEnt {
    s: u64,
    b1: i8,
    b2: i8,
}

static MOVE: [[i8; 5]; 13] = [
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

fn lookup(hash: &[HashEnt], s: u64) -> i32 {
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

fn swap_state(mut s: u64, p1: i32, sp: i32) -> u64 {
    // Mirrors C exactly: p1 = (12 - p1) << 2, sp = (12 - sp) << 2;
    let p1s = ((12 - p1) << 2) as u32;
    let sps = ((12 - sp) << 2) as u32;
    let n1 = (s >> p1s) & 0xFULL;
    s &= !(0xFULL << p1s);
    s |= n1 << sps;
    s
}

fn sach(
    hash: &mut [HashEnt],
    q: &mut [QueEnt],
    mut s: u64,
    b1: i32,
    b2: i32,
    mode: i32,
    mut step: i32,
    lim: i32,
) -> i32 {
    // Queue with sentinel entries where s==0 marks level boundary.
    let mut top: usize = 0;
    let mut end: usize = 0;

    q[end] = QueEnt {
        s,
        b1: b1 as i8,
        b2: b2 as i8,
    };
    end += 1;
    q[end] = QueEnt { s: 0, b1: 0, b2: 0 };
    end += 1;

    while top < end {
        let cur = q[top];
        top += 1;

        s = cur.s;
        let mut b = [cur.b1 as i32, cur.b2 as i32];

        if s == 0 {
            step += 1;
            // push next level sentinel
            q[end] = QueEnt { s: 0, b1: 0, b2: 0 };
            end += 1;
            continue;
        }
        if step > lim {
            break;
        }

        for i in 0..2 {
            let bi = b[i];
            let mut j = 0usize;
            loop {
                let x = MOVE[bi as usize][j] as i32;
                if x < 0 {
                    break;
                }
                if x == b[0] || x == b[1] {
                    j += 1;
                    continue;
                }
                let s2 = swap_state(s, x, bi);
                if mode != 0 {
                    if lookup(hash, s2) >= 0 {
                        return step;
                    }
                } else if !insert(hash, s2, step as i8) {
                    j += 1;
                    continue;
                }

                if i == 1 {
                    q[end] = QueEnt {
                        s: s2,
                        b1: b[0] as i8,
                        b2: x as i8,
                    };
                } else {
                    q[end] = QueEnt {
                        s: s2,
                        b1: x as i8,
                        b2: b[1] as i8,
                    };
                }
                end += 1;
                j += 1;
            }
        }
    }
    step
}

fn main() {
    // Read all input; emulate scanf("%d") tokenization.
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

    let lim: i32 = 16;

    // Precompute from goal state.
    let goal: u64 = 0x123456789ab0u64;
    insert(&mut hash, goal, 0);
    sach(&mut hash, &mut q, goal, 0, 12, 0, 1, lim);

    let mut out = String::new();

    loop {
        let mut b1: i32 = -1;
        let mut b2: i32 = -1;
        let mut s: u64 = 0;

        for i in 0..13 {
            let tok = match it.next() {
                Some(t) => t,
                None => {
                    print!("{out}");
                    return;
                }
            };
            let p: i32 = tok.parse().unwrap();

            if i == 0 && p < 0 {
                print!("{out}");
                return;
            }
            if p == 0 {
                if b1 < 0 {
                    b1 = i as i32;
                } else {
                    b2 = i as i32;
                }
            }
            s = (s << 4) | (p as u64);
        }

        let mut ans = lookup(&hash, s);
        if ans < 0 {
            let mut t = sach(&mut hash, &mut q, s, b1, b2, 1, 1, 4);
            if t > 4 {
                ans = -1;
            } else {
                t += lim;
                ans = t;
            }
        }

        if ans < 0 {
            out.push_str("NA\n");
        } else {
            out.push_str(&format!("{ans}\n"));
        }
    }
}
