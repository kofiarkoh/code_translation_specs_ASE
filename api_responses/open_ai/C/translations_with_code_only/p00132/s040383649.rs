// translated from the given C program to Rust
// Preserves semantics of the original C code (I/O formatting, floating behavior, rotation logic).
use std::io::{self, Read};
use std::cmp::Ordering;

#[derive(Clone)]
struct Order {
    s: usize,
    bx: usize,
    by: usize,
    angle: usize,
    rate: f64,
}

struct Game {
    H: usize,
    W: usize,
    field: Vec<Vec<char>>,
    puzzle: Vec<Vec<char>>,
    n: usize,
    h0: Vec<[usize; 4]>,
    w0: Vec<[usize; 4]>,
    // piece[s][a][y][x]
    piece: Vec<Vec<Vec<Vec<char>>>>,
    k: usize,
    t: Vec<usize>,
}

impl Game {
    fn bury(&mut self, u: &Order) {
        let s = u.s;
        let a = u.angle;
        let h = self.h0[s][a];
        let w = self.w0[s][a];
        for y in 0..h {
            for x in 0..w {
                if self.piece[s][a][y][x] == '#' {
                    let yy = u.by + y;
                    let xx = u.bx + x;
                    self.field[yy][xx] = '*';
                }
            }
        }
    }

    fn fitin(&self, u: &Order, a: usize, bx: i32, by: i32) -> f64 {
        let s = u.s;
        let h = self.h0[s][a] as i32;
        let w = self.w0[s][a] as i32;

        let dx = [0, 1, 0, -1];
        let dy = [1, 0, -1, 0];

        let mut count: i32 = 0;
        let mut total: i32 = 0;

        for x in -1..=(w) {
            let nx = bx + x;
            for y in -1..=(h) {
                let ny = by + y;
                if (0 <= x && x < w) && (0 <= y && y < h)
                    && self.piece[s][a][y as usize][x as usize] == '#'
                {
                    if (0 <= nx && nx < self.W as i32) && (0 <= ny && ny < self.H as i32)
                        && self.field[ny as usize][nx as usize] != '.'
                    {
                        return -100.0;
                    }
                } else {
                    let mut i = 0;
                    for ii in 0..4 {
                        let tx = x + dx[ii];
                        let ty = y + dy[ii];
                        if (0 <= tx && tx < w) && (0 <= ty && ty < h)
                            && self.piece[s][a][ty as usize][tx as usize] == '#'
                        {
                            i = ii + 1; // mark found
                            break;
                        }
                    }
                    if i != 0 {
                        total += 1;
                        if nx < 0
                            || nx >= self.W as i32
                            || ny < 0
                            || ny >= self.H as i32
                            || self.field[ny as usize][nx as usize] != '.'
                        {
                            count += 1;
                        }
                    }
                }
            }
        }

        // preserve C's floating behavior: division by zero may produce inf/NaN
        (count as f64) * 100.0 / (total as f64)
    }

    fn probe(&mut self, u: &mut Order) {
        let s = u.s;
        for a in 0..4 {
            let h = self.h0[s][a];
            let w = self.w0[s][a];

            // find first '#' in first row (row 0). In C code this used strchr and would be UB if missing;
            // here we expect pieces to have at least one '#'.
            let fx = match self.piece[s][a][0].iter().position(|&c| c == '#') {
                Some(pos) => pos,
                None => 0, // fallback; original C would be UB if missing - inputs are assumed valid.
            };

            if h > self.H || w > self.W {
                continue;
            }

            for by in 0..=self.H - h {
                for bx in 0..=self.W - w {
                    if self.field[by][bx + fx] == '.' {
                        let r = self.fitin(u, a, bx as i32, by as i32);
                        if r > u.rate {
                            u.rate = r;
                            u.bx = bx;
                            u.by = by;
                            u.angle = a;
                            if u.rate >= 100.0 {
                                // bury directly as in C
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
        // copy puzzle to field
        self.field = self.puzzle.clone();

        let mut order: Vec<Order> = Vec::with_capacity(self.k);
        let mut memo: Vec<usize> = Vec::with_capacity(self.k);

        // initial memo from t[0..k-1]
        memo.clear();
        for i in 0..self.k {
            memo.push(self.t[i]);
        }

        while !memo.is_empty() {
            let mut orders_this_round: Vec<Order> = Vec::new();

            for &ms in &memo {
                let mut u = Order {
                    s: ms,
                    bx: 0,
                    by: 0,
                    angle: 0,
                    rate: -100.0,
                };
                self.probe(&mut u);
                // In C, they check u.rate != 100.0; pieces that achieved exactly 100 were already buried by probe.
                if u.rate != 100.0 {
                    orders_this_round.push(u);
                }
            }

            if orders_this_round.is_empty() {
                break;
            }

            // sort descending by rate, preserving C comparator behavior: if comparisons with NaN, both < and > are false -> equal
            orders_this_round.sort_by(|l, r| {
                if l.rate < r.rate {
                    Ordering::Greater
                } else if l.rate > r.rate {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });

            if orders_this_round[0].rate == -100.0 {
                return false;
            }

            // bury best
            let best = &orders_this_round[0];
            self.bury(best);

            // memo becomes the remaining orders' s
            memo.clear();
            for ord in orders_this_round.iter().skip(1) {
                memo.push(ord.s);
            }
        }

        // check for any '.' left
        for y in 0..self.H {
            for x in 0..self.W {
                if self.field[y][x] == '.' {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    // Read entire stdin and tokenize by whitespace (matches scanf("%s") semantics for strings with no spaces)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let H_str = match it.next() {
            Some(s) => s,
            None => break,
        };
        let W_str = match it.next() {
            Some(s) => s,
            None => break,
        };
        let H: usize = H_str.parse().unwrap();
        let W: usize = W_str.parse().unwrap();
        if H == 0 && W == 0 {
            break;
        }

        // read puzzle
        let mut puzzle: Vec<Vec<char>> = vec![vec!['.'; W]; H];
        for i in 0..H {
            let row = it.next().expect("expected puzzle row");
            let chars: Vec<char> = row.chars().collect();
            // scanf("%s") would read up to null; we assume provided strings are of length W
            for x in 0..W {
                puzzle[i][x] = chars[x];
            }
        }

        let n: usize = it.next().unwrap().parse().unwrap();

        // prepare piece containers
        let mut h0: Vec<[usize; 4]> = vec![[0; 4]; n];
        let mut w0: Vec<[usize; 4]> = vec![[0; 4]; n];
        // piece[s][a] -> Vec<Vec<char>>
        let mut piece: Vec<Vec<Vec<Vec<char>>>> = Vec::with_capacity(n);

        for s in 0..n {
            let h: usize = it.next().unwrap().parse().unwrap();
            let w: usize = it.next().unwrap().parse().unwrap();
            h0[s][0] = h;
            w0[s][0] = w;

            // read base rotation 0
            let mut p0: Vec<Vec<char>> = vec![vec!['.'; w]; h];
            for y in 0..h {
                let row = it.next().unwrap();
                let chars: Vec<char> = row.chars().collect();
                for x in 0..w {
                    p0[y][x] = chars[x];
                }
            }

            // rotation 1: dims w x h
            let mut p1: Vec<Vec<char>> = vec![vec!['.'; h]; w];
            for y in 0..h {
                for x in 0..w {
                    p1[x][h - y - 1] = p0[y][x];
                }
            }
            h0[s][1] = w;
            w0[s][1] = h;

            // rotation 2: dims h x w
            let mut p2: Vec<Vec<char>> = vec![vec!['.'; w]; h];
            for y in 0..h {
                for x in 0..w {
                    p2[h - 1 - y][w - 1 - x] = p0[y][x];
                }
            }
            h0[s][2] = h;
            w0[s][2] = w;

            // rotation 3: dims w x h
            let mut p3: Vec<Vec<char>> = vec![vec!['.'; h]; w];
            for y in 0..h {
                for x in 0..w {
                    p3[w - 1 - x][y] = p0[y][x];
                }
            }
            h0[s][3] = w;
            w0[s][3] = h;

            piece.push(vec![p0, p1, p2, p3]);
        }

        let p: usize = it.next().unwrap().parse().unwrap();
        for _ in 0..p {
            let k: usize = it.next().unwrap().parse().unwrap();
            let mut t: Vec<usize> = vec![0; k];
            for j in 0..k {
                let val: isize = it.next().unwrap().parse().unwrap();
                // input is 1-based, C code did t[j]--; so subtract 1
                t[j] = (val - 1) as usize;
            }

            let mut game = Game {
                H,
                W,
                field: vec![vec!['.'; W]; H],
                puzzle: puzzle.clone(),
                n,
                h0: h0.clone(),
                w0: w0.clone(),
                piece: piece.clone(),
                k,
                t,
            };

            let res = game.solve();
            if res {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
