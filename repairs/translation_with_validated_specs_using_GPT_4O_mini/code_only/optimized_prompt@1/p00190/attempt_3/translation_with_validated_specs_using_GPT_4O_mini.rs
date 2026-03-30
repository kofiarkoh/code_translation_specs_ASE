use std::io::{self, BufRead};
use std::sync::Mutex;

const MAX: usize = 500000;
const HASHSIZ: usize = 900001;

#[derive(Default, Copy, Clone)]
struct Hash {
    s: u64,
    m: i8,
}

#[derive(Default, Copy, Clone)]
struct Queue {
    s: u64,
    b1: i8,
    b2: i8,
}

// Use Mutex to safely share mutable state across threads
lazy_static::lazy_static! {
    static ref HASH: Mutex<[Hash; HASHSIZ + 5]> = Mutex::new([Hash::default(); HASHSIZ + 5]);
    static ref Q: Mutex<[Queue; MAX + 5]> = Mutex::new([Queue::default(); MAX + 5]);
}

fn lookup(s: u64) -> i8 {
    let hash = HASH.lock().unwrap();
    let mut p = (s as usize) % HASHSIZ;
    while hash[p].s != 0 {
        if hash[p].s == s {
            return hash[p].m;
        }
        p += 1;
        if p == HASHSIZ {
            p = 0;
        }
    }
    -1
}

fn insert(s: u64, m: i8) -> i8 {
    let mut hash = HASH.lock().unwrap();
    let mut p = (s as usize) % HASHSIZ;
    while hash[p].s != 0 {
        if hash[p].s == s {
            return 0;
        }
        p += 1;
        if p == HASHSIZ {
            p = 0;
        }
    }
    hash[p].s = s;
    hash[p].m = m;
    1
}

const MOVES: [[i8; 5]; 13] = [
    [2, -1, 0, 0, 0],
    [2, 5, -1, 0, 0],
    [0, 1, 3, 6, -1],
    [2, 7, -1, 0, 0],
    [5, -1, 0, 0, 0],
    [1, 4, 6, 9, -1],
    [2, 5, 7, 10, -1],
    [3, 6, 8, 11, -1],
    [7, -1, 0, 0, 0],
    [5, 10, -1, 0, 0],
    [6, 9, 11, 12, -1],
    [7, 10, -1, 0, 0],
    [10, -1, 0, 0, 0],
];

fn swap(s: u64, p1: i8, sp: i8) -> u64 {
    let n1 = (s >> ((12 - p1) << 2)) & 0xf;
    let mut s = s & !(0xf << ((12 - p1) << 2));
    s |= n1 << ((12 - sp) << 2);
    s
}

fn sach(mut s: u64, b1: i8, b2: i8, mode: i8, mut step: i8, lim: i8) -> i8 {
    let mut q = Q.lock().unwrap();
    let mut top = 0;
    let mut end = 0;

    q[end].s = s;
    q[end].b1 = b1;
    q[end].b2 = b2;
    end += 1;
    q[end].s = 0;
    end += 1;

    while top < end {
        s = q[top].s;
        let b = [q[top].b1, q[top].b2];
        top += 1;

        if s == 0 {
            step += 1;
            q[end].s = 0;
            end += 1;
            continue;
        }
        if step > lim {
            break;
        }

        for i in 0..2 {
            for j in 0..5 {
                let x = MOVES[b[i] as usize][j];
                if x < 0 {
                    break;
                }
                if x == b[0] || x == b[1] {
                    continue;
                }
                let s2 = swap(s, x, b[i]);
                if mode != 0 {
                    if lookup(s2) >= 0 {
                        return step;
                    }
                } else if insert(s2, step) == 0 {
                    continue;
                }
                q[end].s = s2;
                if i != 0 {
                    q[end].b1 = b[0];
                    q[end].b2 = x;
                } else {
                    q[end].b1 = x;
                    q[end].b2 = b[1];
                }
                end += 1;
            }
        }
    }
    step
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = String::new();
    let lim = 16;

    let mut s = 0x123456789ab0;
    insert(s, 0);
    sach(s, 0, 12, 0, 1, lim);

    loop {
        let mut b1 = -1;
        let mut b2 = -1;
        s = 0;

        for i in 0..13 {
            input.clear();
            reader.read_line(&mut input).unwrap();
            let p: i8 = input.trim().parse().unwrap();
            if i == 0 && p < 0 {
                return;
            }
            if p == 0 {
                if b1 < 0 {
                    b1 = i;
                } else {
                    b2 = i;
                }
            }
            s = (s << 4) | (p as u64);
        }

        let mut ans = lookup(s);
        if ans < 0 {
            ans = sach(s, b1, b2, 1, 1, 4);
            if ans > 4 {
                ans = -1;
            } else {
                ans += lim;
            }
        }
        if ans < 0 {
            println!("NA");
        } else {
            println!("{}", ans);
        }
    }
}
