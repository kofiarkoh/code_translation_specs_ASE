// Aizu Vol-1 0190: Eleven Puzzle
// 2017.8.23 bal4u@uu

const MAX: usize = 500000;
const HASHSIZ: u64 = 900001;

#[derive(Default)]
struct Hash {
    s: u64,
    m: i8,
}

#[derive(Default)]
struct Queue {
    s: u64,
    b1: i8,
    b2: i8,
}

static mut HASH: [Hash; (HASHSIZ + 5) as usize] = [Hash::default(); (HASHSIZ + 5) as usize];
static mut Q: [Queue; MAX + 5] = [Queue::default(); MAX + 5];

static mut TOP: usize = 0;
static mut END: usize = 0;

fn lookup(s: u64) -> i8 {
    unsafe {
        let mut p = (s % HASHSIZ) as usize;
        while HASH[p].s != 0 {
            if HASH[p].s == s {
                return HASH[p].m;
            }
            p += 1;
            if p >= HASH.len() {
                p = 0;
            }
        }
    }
    -1
}

fn insert(s: u64, m: i8) -> i8 {
    unsafe {
        let mut p = (s % HASHSIZ) as usize;
        while HASH[p].s != 0 {
            if HASH[p].s == s {
                return 0;
            }
            p += 1;
            if p >= HASH.len() {
                p = 0;
            }
        }
        HASH[p].s = s;
        HASH[p].m = m;
    }
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
    let n1;
    let p1 = (12 - p1) << 2;
    let sp = (12 - sp) << 2;
    n1 = (s >> p1) & 0xf;
    s & !(0xf << p1) | (n1 << sp)
}

fn sach(s: u64, b1: i8, b2: i8, mode: i8, step: i8, lim: i8) -> i8 {
    unsafe {
        let mut b = [0; 2];
        let mut s2;

        TOP = 0;
        END = 0;
        Q[END].s = s;
        Q[END].b1 = b1;
        Q[END].b2 = b2;
        END += 1;
        Q[END].s = 0;
        END += 1;

        while TOP < END {
            s = Q[TOP].s;
            b[0] = Q[TOP].b1;
            b[1] = Q[TOP].b2;
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
                    let x = MOVES[b[i] as usize][j];
                    if x < 0 {
                        break;
                    }
                    if x == b[0] || x == b[1] {
                        continue;
                    }
                    s2 = swap(s, x, b[i]);
                    if mode != 0 {
                        if lookup(s2) >= 0 {
                            return step;
                        }
                    } else if insert(s2, step) == 0 {
                        continue;
                    }
                    Q[END].s = s2;
                    if i != 0 {
                        Q[END].b1 = b[0];
                        Q[END].b2 = x;
                    } else {
                        Q[END].b1 = x;
                        Q[END].b2 = b[1];
                    }
                    END += 1;
                }
            }
        }
        step
    }
}

fn main() {
    let mut ans: i8;
    let mut p: i32;
    let mut b1: i8;
    let mut b2: i8;
    let mut s: u64;
    let lim = 16;

    s = 0x123456789ab0;
    insert(s, 0);
    sach(s, 0, 12, 0, 1, lim);

    loop {
        b1 = -1;
        s = 0;
        for i in 0..13 {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            p = input.trim().parse().unwrap();
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
        ans = lookup(s);
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
