use std::io::{self, Read};

const MAX: usize = 500000;
const HASHSIZ: usize = 900001;

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

fn swap(mut s: u64, p1: i32, sp: i32) -> u64 {
    // Mirrors the C bit-twiddling exactly.
    let p1 = ((12 - p1) << 2) as u32;
    let sp = ((12 - sp) << 2) as u32;
    let n1 = (s >> p1) & 0xF;
    s &= !(0xFULL << p1);
    s |= n1 << sp;
    s
}

fn sach(
    hash: &mut [HashEnt],
    q: &mut [QueEnt],
    s0: u64,
    b1: i32,
    b2: i32,
    mode: i32,
    mut step: i8,
    lim: i8,
) -> i8 {
    let mut top: usize = 0;
    let mut end: usize = 0;

    q[end] = QueEnt {
        s: s0,
        b1: b1 as i8,
        b2: b2 as i8,
    };
    end += 1;
    q[end] = QueEnt { s: 0, b1: 0, b2: 0 };
    end += 1;

    while top < end {
        let cur = q[top];
        top += 1;

        let s = cur.s;
        let b = [cur.b1 as i32, cur.b2 as i32];

        if s == 0 {
            step = step.wrapping_add(1); // C 'char' overflow behavior (though not reached here)
            q[end] = QueEnt { s: 0, b1: 0, b2: 0 };
            end += 1;
            continue;
        }
        if step > lim {
            break;
        }

        for i in 0..2 {
            let bi = b[i];
            for &x8 in MOVE[bi as usize].iter() {
                if x8 < 0 {
                    break;
                }
                let x = x8 as i32;
                if x == b[0] || x == b[1] {
                    continue;
                }
                let s2 = swap(s, x, bi);
                if mode != 0 {
                    if lookup(hash, s2) >= 0 {
                        return step;
                    }
                } else if !insert(hash, s2, step) {
                    continue;
                }

                // Queue capacity is assumed sufficient as in the C code.
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
            }
        }
    }
    step
}

fn main() {
    // Read all input as whitespace-separated integers (like repeated scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut hash = vec![HashEnt { s: 0, m: 0 }; HASHSIZ + 5];
    let mut q = vec![QueEnt { s: 0, b1: 0, b2: 0 }; MAX + 5];

    let lim: i8 = 16;

    // Precompute from goal state.
    let s_goal: u64 = 0x123456789ab0u64;
    insert(&mut hash, s_goal, 0);
    let _ = sach(&mut hash, &mut q, s_goal, 0, 12, 0, 1, lim);

    let mut out = String::new();

    loop {
        let mut b1: i32 = -1;
        let mut b2: i32 = -1;
        let mut s: u64 = 0;

        for i in 0..13 {
            let p_opt = it.next();
            if p_opt.is_none() {
                // Matches behavior of failing scanf: program would stop producing output.
                print!("{out}");
                return;
            }
            let p: i32 = p_opt.unwrap().parse().unwrap();

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
}
