use std::collections::HashMap;
use std::io::{self, BufRead};

const MAX: usize = 500000;
const HASHSIZ: usize = 900001;

type UL = u64;

#[derive(Clone)]
struct HashEntry {
    state: UL,
    moves: i8,
}

struct QueueEntry {
    state: UL,
    b1: i8,
    b2: i8,
}

static mut HASH: [Option<HashEntry>; HASHSIZ + 5] = [None; HASHSIZ + 5];
static mut Q: [QueueEntry; MAX + 5] = [QueueEntry { state: 0, b1: 0, b2: 0 }; MAX + 5];
static mut top: usize = 0;
static mut end: usize = 0;

const MOVES: [[i8; 5]; 13] = [
    [2, -1],
    [2, 5, -1],
    [0, 1, 3, 6, -1],
    [2, 7, -1],
    [5, -1],
    [1, 4, 6, 9, -1],
    [2, 5, 7, 10, -1],
    [3, 6, 8, 11, -1],
    [7, -1],
    [5, 10, -1],
    [6, 9, 11, 12, -1],
    [7, 10, -1],
    [10, -1],
];

fn lookup(s: UL) -> i8 {
    let index = (s % HASHSIZ as UL) as usize;
    let mut p = index;

    loop {
        if let Some(entry) = unsafe { HASH[p] } {
            if entry.state == s {
                return entry.moves;
            }
        } else {
            return -1;
        }
        p = (p + 1) % HASHSIZ;
    }
}

fn insert(s: UL, m: i8) -> bool {
    let index = (s % HASHSIZ as UL) as usize;
    let mut p = index;

    loop {
        if unsafe { HASH[p] }.is_none() {
            unsafe {
                HASH[p] = Some(HashEntry { state: s, moves: m });
            }
            return true;
        } else if unsafe { HASH[p] }.as_ref().unwrap().state == s {
            return false;
        }
        p = (p + 1) % HASHSIZ;
    }
}

fn swap(s: UL, p1: i8, sp: i8) -> UL {
    let n1 = (s >> ((12 - p1) << 2)) & 0xf;
    let mut s = s & !(0xf << ((12 - p1) << 2));
    s |= n1 << ((12 - sp) << 2);
    s
}

fn sach(s: UL, b1: i8, b2: i8, mode: i8, step: i32, lim: i32) -> i32 {
    unsafe {
        top = 0;
        end = 0;
        Q[end] = QueueEntry { state: s, b1, b2 };
        end += 1;

        while top < end {
            let current = Q[top];
            s = current.state;
            let b = [current.b1, current.b2];
            top += 1;

            if s == 0 {
                return step + 1;
            }
            if step > lim {
                break;
            }

            for i in 0..2 {
                for j in 0..MOVES[b[i] as usize].len() {
                    let x = MOVES[b[i] as usize][j];
                    if x < 0 {
                        break;
                    }
                    if x == b[0] || x == b[1] {
                        continue;
                    }
                    let s2 = swap(s, x, b[i]);
                    if mode == 1 && lookup(s2) >= 0 {
                        return step;
                    } else if mode == 0 && !insert(s2, step as i8) {
                        continue;
                    }
                    Q[end] = QueueEntry {
                        state: s2,
                        b1: if i == 1 { b[0] } else { x },
                        b2: if i == 1 { x } else { b[1] },
                    };
                    end += 1;
                }
            }
        }
    }
    step
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = Vec::new();

    for line in handle.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        input.extend(numbers);
    }

    let mut s: UL = 0;
    let mut b1 = -1;
    let mut b2 = -1;

    for (i, &p) in input.iter().enumerate() {
        if i == 0 && p < 0 {
            return;
        }
        if p == 0 {
            if b1 < 0 {
                b1 = i as i8;
            } else {
                b2 = i as i8;
            }
        }
        s = (s << 4) | (p as UL);
    }

    let mut ans = lookup(s);
    if ans < 0 {
        ans = sach(s, b1, b2, 1, 1, 4);
        if ans > 4 {
            ans = -1;
        } else {
            ans += 16; // lim
        }
    }

    if ans < 0 {
        println!("NA");
    } else {
        println!("{}", ans);
    }
}
