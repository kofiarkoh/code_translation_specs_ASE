// Aizu Vol-1 0190: Eleven Puzzle
// 2017.8.23 bal4u@uu

const MAX: usize = 500000;
type UL = u64;

#[derive(Default, Copy, Clone)]
struct Hash {
    s: UL,
    m: u8,
}

#[derive(Default, Copy, Clone)]
struct Que {
    s: UL,
    b1: u8,
    b2: u8,
}

const HASHSIZ: usize = 900001;
static mut HASH: [Hash; HASHSIZ + 5] = [Hash::default(); HASHSIZ + 5];
static mut Q: [Que; MAX + 5] = [Que::default(); MAX + 5];
static mut TOP: usize = 0;
static mut END: usize = 0;

fn lookup(s: UL) -> i32 {
    unsafe {
        let mut p = (s % HASHSIZ as UL) as usize;
        while HASH[p].s != 0 {
            if HASH[p].s == s {
                return HASH[p].m as i32;
            }
            p += 1;
            if p == HASHSIZ {
                p = 0;
            }
        }
    }
    -1
}

fn insert(s: UL, m: u8) -> i32 {
    unsafe {
        let mut p = (s % HASHSIZ as UL) as usize;
        while HASH[p].s != 0 {
            if HASH[p].s == s {
                return 0;
            }
            p += 1;
            if p == HASHSIZ {
                p = 0;
            }
        }
        HASH[p].s = s;
        HASH[p].m = m;
    }
    1
}

const MOVE: [[i32; 5]; 13] = [
    [2, -1, -1, -1, -1],
    [2, 5, -1, -1, -1],
    [0, 1, 3, 6, -1],
    [2, 7, -1, -1, -1],
    [5, -1, -1, -1, -1],
    [1, 4, 6, 9, -1],
    [2, 5, 7, 10, -1],
    [3, 6, 8, 11, -1],
    [7, -1, -1, -1, -1],
    [5, 10, -1, -1, -1],
    [6, 9, 11, 12, -1],
    [7, 10, -1, -1, -1],
    [10, -1, -1, -1, -1],
];

fn swap(s: UL, p1: usize, sp: usize) -> UL {
    let n1 = (s >> ((12 - p1) << 2)) & 0xf;
    let mut s = s & !(0xf << ((12 - p1) << 2));
    s |= n1 << ((12 - sp) << 2);
    s
}

fn sach(s: UL, b1: usize, b2: usize, mode: i32, step: i32, lim: i32) -> i32 {
    let mut b = [0; 2];
    let mut s2;

    unsafe {
        TOP = 0;
        END = 0;
        Q[END].s = s;
        Q[END].b1 = b1 as u8;
        Q[END].b2 = b2 as u8;
        END += 1;
        Q[END].s = 0;
        END += 1;

        while TOP < END {
            s = Q[TOP].s;
            b[0] = Q[TOP].b1 as usize;
            b[1] = Q[TOP].b2 as usize;
            TOP += 1;

            if s == 0 {
                step += 1;
                Q[END].s = 0;
                END += 1;
                continue;
            }
            if step > lim {
                break;
            }

            for i in 0..2 {
                for j in 0..5 {
                    let x = MOVE[b[i]][j];
                    if x < 0 {
                        break;
                    }
                    if x == b[0] || x == b[1] {
                        continue;
                    }
                    s2 = swap(s, x as usize, b[i]);
                    if mode != 0 {
                        if lookup(s2) >= 0 {
                            return step;
                        }
                    } else if insert(s2, step as u8) == 0 {
                        continue;
                    }
                    Q[END].s = s2;
                    if i != 0 {
                        Q[END].b1 = b[0] as u8;
                        Q[END].b2 = x as u8;
                    } else {
                        Q[END].b1 = x as u8;
                        Q[END].b2 = b[1] as u8;
                    }
                    END += 1;
                }
            }
        }
    }
    step
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();
    let mut ans: i32;
    let mut p: i32;
    let mut b1: i32;
    let mut b2: i32;
    let lim = 16;

    let s: UL = 0x123456789ab0;
    insert(s, 0);

    sach(s, 0, 12, 0, 1, lim);

    loop {
        b1 = -1;
        let mut s: UL = 0;

        for i in 0..13 {
            buffer.clear();
            input.read_line(&mut buffer).unwrap();
            p = buffer.trim().parse().unwrap();
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
            s = (s << 4) | (p as UL);
        }

        ans = lookup(s);
        if ans < 0 {
            ans = sach(s, b1 as usize, b2 as usize, 1, 1, 4);
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
