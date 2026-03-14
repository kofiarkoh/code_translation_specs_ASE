use std::io::{self, BufRead};
use std::collections::VecDeque;

const MAX_W: usize = 20;
const MAX_H: usize = 20;
const MAX_N: usize = 10;

#[derive(Clone, Copy)]
struct Order {
    s: usize,
    bx: usize,
    by: usize,
    angle: usize,
    rate: f64,
}

struct PuzzleSolver {
    h: usize,
    w: usize,
    field: [[char; MAX_W + 1]; MAX_H],
    puzzle: [[char; MAX_W + 1]; MAX_H],
    n: usize,
    h0: [[usize; 4]; MAX_N],
    w0: [[usize; 4]; MAX_N],
    piece: [[[char; MAX_W + 1]; MAX_H]; MAX_N],
    k: usize,
    t: [usize; MAX_N],
}

impl PuzzleSolver {
    fn new() -> Self {
        Self {
            h: 0,
            w: 0,
            field: [['.'; MAX_W + 1]; MAX_H],
            puzzle: [['.'; MAX_W + 1]; MAX_H],
            n: 0,
            h0: [[0; 4]; MAX_N],
            w0: [[0; 4]; MAX_N],
            piece: [[['.'; MAX_W + 1]; MAX_H]; MAX_N],
            k: 0,
            t: [0; MAX_N],
        }
    }

    fn bury(&mut self, u: &Order) {
        let s = u.s;
        let a = u.angle;
        let h = self.h0[s][a];
        let w = self.w0[s][a];

        for y in 0..h {
            for x in 0..w {
                if self.piece[s][a][y][x] == '#' {
                    self.field[u.by + y][u.bx + x] = '*';
                }
            }
        }
    }

    fn fitin(&self, u: &Order, a: usize, bx: usize, by: usize) -> f64 {
        let s = u.s;
        let h = self.h0[s][a];
        let w = self.w0[s][a];

        let dx = [0, 1, 0, -1];
        let dy = [1, 0, -1, 0];

        let mut count = 0;
        let mut total = 0;

        for x in -1..=w as isize {
            let nx = bx as isize + x;
            for y in -1..=h as isize {
                let ny = by as isize + y;
                if (0..w as isize).contains(&x) && (0..h as isize).contains(&y) && self.piece[s][a][y as usize][x as usize] == '#' {
                    if !(0..self.w as isize).contains(&nx) || !(0..self.h as isize).contains(&ny) || self.field[ny as usize][nx as usize] != '.' {
                        return -100.0;
                    }
                } else {
                    let mut i = 0;
                    while i < 4 {
                        let tx = x + dx[i];
                        let ty = y + dy[i];
                        if (0..w as isize).contains(&tx) && (0..h as isize).contains(&ty) && self.piece[s][a][ty as usize][tx as usize] == '#' {
                            break;
                        }
                        i += 1;
                    }

                    if i < 4 {
                        total += 1;
                        if nx < 0 || nx >= self.w as isize || ny < 0 || ny >= self.h as isize || self.field[ny as usize][nx as usize] != '.' {
                            count += 1;
                        }
                    }
                }
            }
        }

        if total == 0 {
            return 0.0;
        }
        count as f64 * 100.0 / total as f64
    }

    fn probe(&mut self, u: &mut Order) {
        let s = u.s;
        for a in 0..4 {
            let h = self.h0[s][a];
            let w = self.w0[s][a];

            let fx = self.piece[s][a][0].iter().position(|&c| c == '#').unwrap_or(0);
            for by in 0..=self.h - h {
                for bx in 0..=self.w - w {
                    if self.field[by][bx + fx] == '.' {
                        let r = self.fitin(u, a, bx, by);
                        if r > u.rate {
                            u.rate = r;
                            u.bx = bx;
                            u.by = by;
                            u.angle = a;

                            if u.rate >= 100.0 {
                                self.bury(u);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    fn solve(&mut self) -> bool {
        let mut order = vec![Order { s: 0, bx: 0, by: 0, angle: 0, rate: -100.0 }; MAX_N];
        let mut memo = vec![0; MAX_N];
        let mut ix = 0;

        self.field = self.puzzle;

        for i in 0..self.k {
            memo[ix] = self.t[i];
            ix += 1;
        }

        while ix != 0 {
            let mut ix2 = 0;
            for i in 0..ix {
                let mut u = Order { s: memo[i], bx: 0, by: 0, angle: 0, rate: -100.0 };
                self.probe(&mut u);
                if u.rate != 100.0 {
                    order[ix2] = u;
                    ix2 += 1;
                }
            }

            if ix2 == 0 {
                break;
            }

            order[0..ix2].sort_by(|l, r| r.rate.partial_cmp(&l.rate).unwrap());

            if order[0].rate == -100.0 {
                return false;
            }

            self.bury(&order[0]);

            ix = 0;
            for i in 1..ix2 {
                memo[ix] = order[i].s;
                ix += 1;
            }
        }

        for y in 0..self.h {
            for x in 0..self.w {
                if self.field[y][x] == '.' {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    let mut solver = PuzzleSolver::new();

    loop {
        let dimensions: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        solver.h = dimensions[0];
        solver.w = dimensions[1];

        if solver.h == 0 && solver.w == 0 {
            break;
        }

        for i in 0..solver.h {
            let line: Vec<char> = lines.next().unwrap().chars().collect();
            solver.puzzle[i][0..solver.w].copy_from_slice(&line[0..solver.w]);
        }

        solver.n = lines.next().unwrap().parse().unwrap();
        for s in 0..solver.n {
            let sizes: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            let h = sizes[0];
            let w = sizes[1];

            for y in 0..h {
                let line: Vec<char> = lines.next().unwrap().chars().collect();
                solver.piece[s][0][y][0..w].copy_from_slice(&line[0..w]);
            }

            solver.h0[s][1] = w;
            solver.w0[s][1] = h;
            for y in 0..h {
                for x in 0..w {
                    solver.piece[s][1][x][h - y - 1] = solver.piece[s][0][y][x];
                }
            }

            solver.h0[s][2] = h;
            solver.w0[s][2] = w;
            for y in 0..h {
                for x in 0..w {
                    solver.piece[s][2][h - 1 - y][w - 1 - x] = solver.piece[s][0][y][x];
                }
            }

            solver.w0[s][3] = h;
            solver.h0[s][3] = w;
            for y in 0..h {
                for x in 0..w {
                    solver.piece[s][3][w - 1 - x][y] = solver.piece[s][0][y][x];
                }
            }
        }

        let p: usize = lines.next().unwrap().parse().unwrap();
        for _ in 0..p {
            solver.k = lines.next().unwrap().parse().unwrap();
            for j in 0..solver.k {
                solver.t[j] = lines.next().unwrap().parse::<usize>().unwrap() - 1;
            }

            println!("{}", if solver.solve() { "YES" } else { "NO" });
        }
    }
}
