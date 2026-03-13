// translated from the provided C program per specification.
// Safe, idiomatic Rust; preserves semantics of the original greedy placement algorithm.

use std::io::{self, Read};
use std::cmp::Ordering;

const MAX_W: usize = 20;
const MAX_H: usize = 20;
const MAX_N: usize = 10;

#[derive(Clone, Debug)]
struct Order {
    s: usize,
    bx: usize,
    by: usize,
    angle: usize,
    rate: f64,
}

struct PuzzleInstance {
    h: usize,
    w: usize,
    puzzle: Vec<Vec<char>>, // H x W
    n: usize,
    // For each piece s in 0..n:
    // h0[s][a], w0[s][a] for a in 0..4
    h0: Vec<[usize; 4]>,
    w0: Vec<[usize; 4]>,
    // piece[s][a] -> Vec<Vec<char>> of size h0[s][a] x w0[s][a]
    piece: Vec<[Vec<Vec<char>>; 4]>,
}

fn bury(field: &mut [Vec<char>], order: &Order, h0: &[ [usize;4] ], w0: &[ [usize;4] ], piece: &Vec<[Vec<Vec<char>>;4]>) {
    let s = order.s;
    let a = order.angle;
    let h = h0[s][a];
    let w = w0[s][a];
    for y in 0..h {
        for x in 0..w {
            if piece[s][a][y][x] == '#' {
                field[order.by + y][order.bx + x] = '*';
            }
        }
    }
}

fn fitin(field: &[Vec<char>], s: usize, a: usize, bx: usize, by: usize,
         H: usize, W: usize,
         h0: &[ [usize;4] ], w0: &[ [usize;4] ],
         piece: &Vec<[Vec<Vec<char>>;4]>) -> f64
{
    let h = h0[s][a];
    let w = w0[s][a];

    let dx = [0isize, 1, 0, -1];
    let dy = [1isize, 0, -1, 0];

    let mut count: i32 = 0;
    let mut total: i32 = 0;

    for x in -1isize..=(w as isize) {
        let nx = bx as isize + x;
        for y in -1isize..=(h as isize) {
            let ny = by as isize + y;
            // inside piece and cell is '#'
            if (0 <= x && x < w as isize) && (0 <= y && y < h as isize) &&
               piece[s][a][y as usize][x as usize] == '#'
            {
                // if placement overlaps an occupied board cell -> invalid
                if 0 <= nx && nx < W as isize && 0 <= ny && ny < H as isize {
                    if field[ny as usize][nx as usize] != '.' {
                        return -100.0;
                    }
                }
            } else {
                // check if this (x,y) is adjacent to any '#' in the piece
                let mut adjacent = false;
                for i in 0..4 {
                    let tx = x + dx[i];
                    let ty = y + dy[i];
                    if (0 <= tx && tx < w as isize) && (0 <= ty && ty < h as isize) {
                        if piece[s][a][ty as usize][tx as usize] == '#' {
                            adjacent = true;
                            break;
                        }
                    }
                }
                if adjacent {
                    total += 1;
                    if nx < 0 || nx >= W as isize || ny < 0 || ny >= H as isize ||
                       field[ny as usize][nx as usize] != '.' {
                        count += 1;
                    }
                }
            }
        }
    }

    if total == 0 {
        // The original C code would divide by zero -> undefined.
        // To avoid panic while keeping semantics conservative, treat as invalid.
        return -100.0;
    }

    (count as f64) * 100.0 / (total as f64)
}

fn probe(u: &mut Order, H: usize, W: usize,
         field: &mut [Vec<char>],
         h0: &[ [usize;4] ], w0: &[ [usize;4] ],
         piece: &Vec<[Vec<Vec<char>>;4]>)
{
    let s = u.s;
    for a in 0..4 {
        let h = h0[s][a];
        let w = w0[s][a];

        // find first '#' in the first row of this rotation
        // Per SPEC, first row contains at least one '#'; replicate behavior by expecting it.
        let first_row = &piece[s][a][0];
        let fx = first_row.iter().position(|&c| c == '#')
            .expect("Piece first row must contain '#' per specification");

        if h > H || w > W {
            continue; // cannot place this rotation at all
        }

        for by in 0..=(H - h) {
            for bx in 0..=(W - w) {
                if field[by][bx + fx] == '.' {
                    let r = fitin(field, s, a, bx, by, H, W, h0, w0, piece);
                    if r > u.rate {
                        u.rate = r;
                        u.bx = bx;
                        u.by = by;
                        u.angle = a;

                        if u.rate >= 100.0 {
                            // place immediately
                            bury(field, u, h0, w0, piece);
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn solve_instance(H: usize, W: usize,
                  puzzle: &[Vec<char>],
                  k: usize,
                  t: &[usize],
                  h0: &[ [usize;4] ], w0: &[ [usize;4] ],
                  piece: &Vec<[Vec<Vec<char>>;4]>)
                  -> bool
{
    let mut field = puzzle.to_vec(); // clone starting board
    let mut memo: Vec<usize> = Vec::with_capacity(k);
    for &idx in t.iter().take(k) {
        memo.push(idx);
    }

    while !memo.is_empty() {
        let mut order: Vec<Order> = Vec::new();
        // iterate through memo pieces
        for &sidx in memo.iter() {
            let mut u = Order { s: sidx, bx: 0, by: 0, angle: 0, rate: -100.0 };
            probe(&mut u, H, W, &mut field, h0, w0, piece);
            // if probe placed it immediately (rate >= 100), skip adding to order
            if (u.rate - 100.0).abs() > std::f64::EPSILON {
                // not exactly 100.0 => add to candidates (the C code uses u.rate != 100.0)
                order.push(u);
            }
            // else: placed immediately and not pushed to order (so effectively removed)
        }

        if order.is_empty() {
            break;
        }

        // sort descending by rate
        order.sort_by(|a, b| {
            // handle partial_cmp; rates are finite
            match b.rate.partial_cmp(&a.rate) {
                Some(ord) => ord,
                None => Ordering::Equal,
            }
        });

        if (order[0].rate - (-100.0)).abs() < std::f64::EPSILON {
            return false;
        }

        // place best candidate
        bury(&mut field, &order[0], h0, w0, piece);

        // rebuild memo from remaining candidates (skip the placed one at index 0)
        memo.clear();
        for o in order.into_iter().skip(1) {
            memo.push(o.s);
        }
    }

    // check if any '.' remains
    for y in 0..H {
        for x in 0..W {
            if field[y][x] == '.' {
                return false;
            }
        }
    }
    true
}

fn main() {
    // Read entire stdin and tokenize by whitespace (matches scanf %s behavior)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed read stdin");
    let mut toks = input.split_whitespace();
    loop {
        let H_opt = toks.next();
        let W_opt = toks.next();
        if H_opt.is_none() || W_opt.is_none() {
            break;
        }
        let H: usize = H_opt.unwrap().parse().expect("H parse");
        let W: usize = W_opt.unwrap().parse().expect("W parse");
        if H == 0 && W == 0 {
            break;
        }

        // read puzzle board H lines
        let mut puzzle: Vec<Vec<char>> = Vec::with_capacity(H);
        for _ in 0..H {
            let row = toks.next().expect("board row");
            let chars: Vec<char> = row.chars().collect();
            // assume proper length as per spec
            puzzle.push(chars);
        }

        // number of distinct pieces
        let n: usize = toks.next().expect("n").parse().expect("n parse");
        // prepare containers
        let mut h0: Vec<[usize;4]> = Vec::with_capacity(n);
        let mut w0: Vec<[usize;4]> = Vec::with_capacity(n);
        let mut piece: Vec<[Vec<Vec<char>>;4]> = Vec::with_capacity(n);

        for _s in 0..n {
            let h_orig: usize = toks.next().expect("h0").parse().expect("h0 parse");
            let w_orig: usize = toks.next().expect("w0").parse().expect("w0 parse");

            // read canonical piece orientation (rotation 0)
            let mut p0: Vec<Vec<char>> = Vec::with_capacity(h_orig);
            for _ in 0..h_orig {
                let prow = toks.next().expect("piece row");
                p0.push(prow.chars().collect());
            }

            // allocate rotations
            // rot0
            let mut rot0 = p0.clone();

            // rot1: dims w x h (rows = w, cols = h)
            let mut rot1: Vec<Vec<char>> = vec![vec!['.'; h_orig]; w_orig];
            for y in 0..h_orig {
                for x in 0..w_orig {
                    // piece[s][1][x][h - y - 1] = piece[s][0][y][x];
                    rot1[x][h_orig - y - 1] = p0[y][x];
                }
            }

            // rot2: dims h x w
            let mut rot2: Vec<Vec<char>> = vec![vec!['.'; w_orig]; h_orig];
            for y in 0..h_orig {
                for x in 0..w_orig {
                    // piece[s][2][h - 1 - y][w - 1 - x] = piece[s][0][y][x];
                    rot2[h_orig - 1 - y][w_orig - 1 - x] = p0[y][x];
                }
            }

            // rot3: dims w x h
            let mut rot3: Vec<Vec<char>> = vec![vec!['.'; h_orig]; w_orig];
            for y in 0..h_orig {
                for x in 0..w_orig {
                    // piece[s][3][w - 1 - x][y] = piece[s][0][y][x];
                    rot3[w_orig - 1 - x][y] = p0[y][x];
                }
            }

            let mut h_arr = [0usize; 4];
            let mut w_arr = [0usize; 4];
            h_arr[0] = h_orig;
            w_arr[0] = w_orig;
            h_arr[1] = w_orig;
            w_arr[1] = h_orig;
            h_arr[2] = h_orig;
            w_arr[2] = w_orig;
            h_arr[3] = w_orig;
            w_arr[3] = h_orig;

            h0.push(h_arr);
            w0.push(w_arr);
            piece.push([rot0, rot1, rot2, rot3]);
        }

        // number of placement queries
        let p: usize = toks.next().expect("p").parse().expect("p parse");

        for _ in 0..p {
            let k: usize = toks.next().expect("k").parse().expect("k parse");
            let mut t: Vec<usize> = Vec::with_capacity(k);
            for _ in 0..k {
                let idx1: usize = toks.next().expect("t").parse().expect("t parse");
                t.push(idx1 - 1); // convert 1-based to 0-based
            }

            let res = solve_instance(H, W, &puzzle, k, &t, &h0, &w0, &piece);
            if res {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
