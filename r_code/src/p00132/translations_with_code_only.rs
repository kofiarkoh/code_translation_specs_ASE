use std::io::{self, BufRead};
use std::mem;
use std::ptr;

const MAX_W: usize = 20;
const MAX_H: usize = 20;
const MAX_N: usize = 10;

#[derive(Debug)]
struct Order {
    s: usize,
    bx: usize,
    by: usize,
    angle: usize,
    rate: f64,
}

static mut H: usize = 0;
static mut W: usize = 0;
static mut FIELD: [[char; MAX_W + 1]; MAX_H] = [['\0'; MAX_W + 1]; MAX_H];
static mut PUZZLE: [[char; MAX_W + 1]; MAX_H] = [['\0'; MAX_W + 1]; MAX_H];
static mut N: usize = 0;
static mut H0: [[usize; 4]; MAX_N] = [[0; 4]; MAX_N];
static mut W0: [[usize; 4]; MAX_N] = [[0; 4]; MAX_N];
static mut PIECE: [[[[char; MAX_W + 1]; MAX_H]; 4]; MAX_N] = [[[['\0'; MAX_W + 1]; MAX_H]; 4]; MAX_N];
static mut K: usize = 0;
static mut T: [usize; MAX_N] = [0; MAX_N];

fn bury(u: &Order) {
    let s = u.s;
    let a = u.angle;
    let h = unsafe { H0[s][a] };
    let w = unsafe { W0[s][a] };

    for y in 0..h {
        for x in 0..w {
            if unsafe { PIECE[s][a][y][x] } == '#' {
                unsafe {
                    FIELD[u.by + y][u.bx + x] = '*';
                }
            }
        }
    }
}

fn fitin(u: &Order, a: usize, bx: usize, by: usize) -> f64 {
    let s = u.s;
    let h = unsafe { H0[s][a] };
    let w = unsafe { W0[s][a] };

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    let mut count = 0;
    let mut total = 0;
    for x in -1..=w as isize {
        let nx = bx as isize + x;
        for y in -1..=h as isize {
            let ny = by as isize + y;
            if (0..w as isize).contains(&x) && (0..h as isize).contains(&y) && unsafe { PIECE[s][a][y as usize][x as usize] } == '#' {
                if (0..W as isize).contains(&nx) && (0..H as isize).contains(&ny) && unsafe { FIELD[ny as usize][nx as usize] } != '.' {
                    return -100.0;
                }
            } else {
                let mut i = 0;
                while i < 4 {
                    let tx = x + dx[i];
                    let ty = y + dy[i];
                    if (0..w as isize).contains(&tx) && (0..h as isize).contains(&ty) && unsafe { PIECE[s][a][ty as usize][tx as usize] } == '#' {
                        break;
                    }
                    i += 1;
                }

                if i < 4 {
                    total += 1;
                    if nx < 0 || nx >= W as isize || ny < 0 || ny >= H as isize || unsafe { FIELD[ny as usize][nx as usize] } != '.' {
                        count += 1;
                    }
                }
            }
        }
    }

    count as f64 * 100.0 / total as f64
}

fn probe(u: &mut Order) {
    let s = u.s;
    for a in 0..4 {
        let h = unsafe { H0[s][a] };
        let w = unsafe { W0[s][a] };

        let fx = unsafe { PIECE[s][a][0].iter().position(|&c| c == '#').unwrap() };
        for by in 0..=H - h {
            for bx in 0..=W - w {
                if unsafe { FIELD[by][bx + fx] } == '.' {
                    let r = fitin(u, a, bx, by);
                    if r > u.rate {
                        u.rate = r;
                        u.bx = bx;
                        u.by = by;
                        u.angle = a;

                        if u.rate >= 100.0 {
                            bury(u);
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn cmp_func(l: &Order, r: &Order) -> std::cmp::Ordering {
    l.rate.partial_cmp(&r.rate).unwrap_or(std::cmp::Ordering::Equal).reverse()
}

fn solve() -> bool {
    let mut u = Order { s: 0, bx: 0, by: 0, angle: 0, rate: -100.0 };
    let mut order: [Order; MAX_N] = Default::default();
    let mut memo: [usize; MAX_N] = Default::default();
    let mut ix = 0;

    unsafe {
        FIELD.copy_from_slice(&PUZZLE);
        for i in 0..K {
            memo[ix] = T[i];
            ix += 1;
        }
    }

    while ix != 0 {
        let mut ix2 = 0;
        for i in 0..ix {
            u = Order { s: memo[i], bx: 0, by: 0, angle: 0, rate: -100.0 };
            probe(&mut u);
            if u.rate != 100.0 {
                order[ix2] = u;
                ix2 += 1;
            }
        }

        if ix2 == 0 {
            break;
        }

        order[0..ix2].sort_by(cmp_func);
        if order[0].rate == -100.0 {
            return false;
        }

        bury(&order[0]);

        ix = 0;
        for i in 1..ix2 {
            memo[ix] = order[i].s;
            ix += 1;
        }
    }

    for y in 0..H {
        for x in 0..W {
            if unsafe { FIELD[y][x] } == '.' {
                return false;
            }
        }
    }

    true
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let h: usize = parts.next().unwrap().parse().unwrap();
        let w: usize = parts.next().unwrap().parse().unwrap();
        if h == 0 && w == 0 {
            break;
        }

        unsafe {
            H = h;
            W = w;
            for i in 0..H {
                let line = reader.lines().next().unwrap().unwrap();
                PUZZLE[i].copy_from_slice(line.as_bytes().iter().map(|&b| b as char).collect::<Vec<_>>().as_slice());
            }

            N = reader.lines().next().unwrap().unwrap().parse().unwrap();
            for s in 0..N {
                let line = reader.lines().next().unwrap().unwrap();
                let mut parts = line.split_whitespace();
                H0[s][0] = parts.next().unwrap().parse().unwrap();
                W0[s][0] = parts.next().unwrap().parse().unwrap();
                let h = H0[s][0];
                let w = W0[s][0];

                for y in 0..h {
                    let line = reader.lines().next().unwrap().unwrap();
                    PIECE[s][0][y].copy_from_slice(line.as_bytes().iter().map(|&b| b as char).collect::<Vec<_>>().as_slice());
                }

                H0[s][1] = w;
                W0[s][1] = h;
                for y in 0..h {
                    for x in 0..w {
                        PIECE[s][1][x][h - y - 1] = PIECE[s][0][y][x];
                    }
                }

                H0[s][2] = h;
                W0[s][2] = w;
                for y in 0..h {
                    for x in 0..w {
                        PIECE[s][2][h - 1 - y][w - 1 - x] = PIECE[s][0][y][x];
                    }
                }

                W0[s][3] = h;
                H0[s][3] = w;
                for y in 0..h {
                    for x in 0..w {
                        PIECE[s][3][w - 1 - x][y] = PIECE[s][0][y][x];
                    }
                }
            }

            let p: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
            for _ in 0..p {
                let k: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
                for j in 0..k {
                    let t_j: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
                    T[j] = t_j - 1;
                }

                println!("{}", if solve() { "YES" } else { "NO" });
            }
        }
    }
}
