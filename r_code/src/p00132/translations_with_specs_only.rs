// translator of the given specification into Rust
// Implements the greedy tiling algorithm described in the specification.
// Treats '.' as empty cell; any other character as occupied.
// Pieces may be rotated by 0,90,180,270 degrees (no reflections).
// For each query, applies the greedy placement heuristic and prints YES/NO.

use std::io::{self, Read};

#[derive(Clone)]
struct Rotation {
    h: usize,
    w: usize,
    grid: Vec<Vec<char>>, // row-major grid of chars, '#' means filled
    filled: Vec<(usize, usize)>, // list of coordinates of '#' cells
    // total perimeter sides (for score denominator)
    total_border_sides: usize,
}

#[derive(Clone)]
struct Piece {
    // store all 4 rotations (r = 0..3)
    rots: [Rotation; 4],
}

fn rotate90(g: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = g.len();
    let w = if h > 0 { g[0].len() } else { 0 };
    let mut ng = vec![vec!['.'; h]; w];
    for i in 0..h {
        for j in 0..w {
            ng[j][h - 1 - i] = g[i][j];
        }
    }
    ng
}

fn build_rotation(grid: &Vec<Vec<char>>) -> Rotation {
    let h = grid.len();
    let w = if h > 0 { grid[0].len() } else { 0 };
    let mut filled = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                filled.push((i, j));
            }
        }
    }
    // compute total border sides: for each filled cell, for each of 4 neighbors
    let mut total_border_sides = 0usize;
    for &(i, j) in &filled {
        // up
        if !(i >= 1 && grid[i - 1][j] == '#') { total_border_sides += 1; }
        // down
        if !(i + 1 < h && grid[i + 1][j] == '#') { total_border_sides += 1; }
        // left
        if !(j >= 1 && grid[i][j - 1] == '#') { total_border_sides += 1; }
        // right
        if !(j + 1 < w && grid[i][j + 1] == '#') { total_border_sides += 1; }
    }

    Rotation {
        h,
        w,
        grid: grid.clone(),
        filled,
        total_border_sides,
    }
}

fn make_piece(canonical: Vec<Vec<char>>) -> Piece {
    let r0 = canonical;
    let r1 = rotate90(&r0);
    let r2 = rotate90(&r1);
    let r3 = rotate90(&r2);
    let rots = [
        build_rotation(&r0),
        build_rotation(&r1),
        build_rotation(&r2),
        build_rotation(&r3),
    ];
    Piece { rots }
}

fn solve_query(
    H: usize,
    W: usize,
    board_in: &Vec<Vec<char>>,
    pieces: &Vec<Piece>,
    mut remaining: Vec<usize>, // list of piece type indices (0-based), possibly with repeats
) -> bool {
    // working board
    let mut board = board_in.clone();

    // quick check helper: any '.' left?
    fn board_full(board: &Vec<Vec<char>>) -> bool {
        for row in board.iter() {
            for &c in row.iter() {
                if c == '.' {
                    return false;
                }
            }
        }
        true
    }

    if board_full(&board) {
        return true;
    }

    // directions for neighbor checks
    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while !remaining.is_empty() {
        // For each remaining piece, find its best placement (score, rot, top-left r,c)
        // If any placement achieves 100, place immediately and continue outer loop.
        let mut best_overall: Option<(usize, i32, usize, usize, usize)> = None;
        // tuple: (index_in_remaining, score, rot_id, r0, c0)
        let mut placed_immediately = false;

        for (idx_in_rem, &ptype) in remaining.iter().enumerate() {
            let piece = &pieces[ptype];
            let mut best_for_piece_score: i32 = i32::MIN;
            let mut best_for_piece_rot: usize = 0;
            let mut best_for_piece_r: usize = 0;
            let mut best_for_piece_c: usize = 0;

            // iterate rotations
            for rot_id in 0..4 {
                let rot = &piece.rots[rot_id];
                if rot.filled.is_empty() {
                    // specification states undefined if no '#' in first row of canonical,
                    // but generally we can ignore pieces without filled cells (treat invalid)
                    continue;
                }
                if rot.h > H || rot.w > W {
                    continue; // can't place at any position
                }
                let max_r0 = H - rot.h;
                let max_c0 = W - rot.w;

                // positions
                for r0 in 0..=max_r0 {
                    for c0 in 0..=max_c0 {
                        // check overlap validity
                        let mut overlap = false;
                        for &(pi, pj) in &rot.filled {
                            let bi = r0 + pi;
                            let bj = c0 + pj;
                            if board[bi][bj] != '.' {
                                overlap = true;
                                break;
                            }
                        }
                        if overlap {
                            // invalid placement -> score -100 as per spec (we will treat any negative as invalid)
                            continue;
                        }

                        // compute border hits and total border sides
                        let mut hits: usize = 0;
                        // total border sides should be rot.total_border_sides (constant), but
                        // count hits by checking neighbors against board/out-of-bounds/occupied.
                        for &(pi, pj) in &rot.filled {
                            for &(di, dj) in &DIRS {
                                let ni_piece = pi as isize + di;
                                let nj_piece = pj as isize + dj;
                                // if neighbor is inside piece and filled, it's not a border side
                                let neighbor_in_piece_filled = if ni_piece >= 0
                                    && (ni_piece as usize) < rot.h
                                    && nj_piece >= 0
                                    && (nj_piece as usize) < rot.w
                                {
                                    rot.grid[ni_piece as usize][nj_piece as usize] == '#'
                                } else {
                                    false
                                };
                                if neighbor_in_piece_filled {
                                    continue;
                                }
                                // otherwise it's a border side -> check if neighbor on board is out-of-bounds or occupied
                                let gi = r0 as isize + pi as isize + di;
                                let gj = c0 as isize + pj as isize + dj;
                                if gi < 0 || gj < 0 || gi >= H as isize || gj >= W as isize {
                                    hits += 1;
                                } else {
                                    let cell = board[gi as usize][gj as usize];
                                    if cell != '.' {
                                        hits += 1;
                                    }
                                }
                            }
                        }

                        let denom = rot.total_border_sides;
                        // denom should be > 0 for a piece with at least one '#'
                        if denom == 0 {
                            // defensive: if denom is zero (shouldn't happen), treat as full contact
                            // so score 100
                            let score = 100i32;
                            if score > best_for_piece_score {
                                best_for_piece_score = score;
                                best_for_piece_rot = rot_id;
                                best_for_piece_r = r0;
                                best_for_piece_c = c0;
                            }
                            if score == 100 {
                                // place immediately
                                // apply placement
                                for &(pi, pj) in &rot.filled {
                                    board[r0 + pi][c0 + pj] = '*';
                                }
                                // remove this piece from remaining
                                remaining.remove(idx_in_rem);
                                placed_immediately = true;
                                break;
                            }
                        } else {
                            let score = ((hits * 100) / denom) as i32;
                            if score > best_for_piece_score {
                                best_for_piece_score = score;
                                best_for_piece_rot = rot_id;
                                best_for_piece_r = r0;
                                best_for_piece_c = c0;
                            }
                            if score == 100 {
                                // place immediately
                                let rot_for_place = &piece.rots[rot_id];
                                for &(pi, pj) in &rot_for_place.filled {
                                    board[r0 + pi][c0 + pj] = '*';
                                }
                                remaining.remove(idx_in_rem);
                                placed_immediately = true;
                                break;
                            }
                        }
                    } // c0
                    if placed_immediately { break; }
                } // r0
                if placed_immediately { break; }
            } // rot_id
            if placed_immediately {
                break; // restart outer while loop for new board state
            }
            // If we had a best_for_piece_score that is valid (>=0), consider it for overall best.
            if best_for_piece_score >= 0 {
                match best_overall {
                    None => {
                        best_overall = Some((
                            idx_in_rem,
                            best_for_piece_score,
                            best_for_piece_rot,
                            best_for_piece_r,
                            best_for_piece_c,
                        ));
                    }
                    Some((best_idx_in_rem, best_score, _, _, _)) => {
                        if best_for_piece_score > best_score {
                            best_overall = Some((
                                idx_in_rem,
                                best_for_piece_score,
                                best_for_piece_rot,
                                best_for_piece_r,
                                best_for_piece_c,
                            ));
                        }
                    }
                }
            }
        } // for each remaining piece

        if placed_immediately {
            // check full
            if board_full(&board) {
                return true;
            } else {
                continue; // continue placing next piece
            }
        }

        // if no piece had any valid placement
        if best_overall.is_none() {
            return false;
        }

        // place the best overall piece (the one with highest score)
        let (idx_in_rem, _score, rot_id, r0, c0) = best_overall.unwrap();
        let ptype = remaining[idx_in_rem];
        let rot = &pieces[ptype].rots[rot_id];
        // sanity: ensure placement is still valid (we checked earlier but board may have changed)
        let mut overlap = false;
        for &(pi, pj) in &rot.filled {
            if board[r0 + pi][c0 + pj] != '.' {
                overlap = true;
                break;
            }
        }
        if overlap {
            // unexpected: if overlap happens due to intermediate placements, treat as no valid placement
            return false;
        }
        for &(pi, pj) in &rot.filled {
            board[r0 + pi][c0 + pj] = '*';
        }
        // remove that piece from remaining
        remaining.remove(idx_in_rem);

        // check if board full
        if board_full(&board) {
            return true;
        }
        // otherwise continue
    } // while remaining not empty

    // no pieces left; success only if board has no '.'
    for row in board.iter() {
        for &c in row.iter() {
            if c == '.' {
                return false;
            }
        }
    }
    true
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().peekable();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        // read H W
        let h_token = match it.next() {
            Some(t) => t,
            None => break,
        };
        let w_token = match it.next() {
            Some(t) => t,
            None => break,
        };
        let H: usize = h_token.parse().unwrap_or(0);
        let W: usize = w_token.parse().unwrap_or(0);
        if H == 0 && W == 0 {
            break;
        }

        // read H board lines
        let mut board: Vec<Vec<char>> = Vec::with_capacity(H);
        for _ in 0..H {
            let line = it.next().unwrap_or("");
            let mut row: Vec<char> = line.chars().collect();
            // if line shorter, pad? Spec assumes proper formatting.
            // If longer, truncate.
            if row.len() < W {
                row.resize(W, '.');
            } else if row.len() > W {
                row.truncate(W);
            }
            board.push(row);
        }

        // read n pieces
        let n_token = it.next().unwrap_or("0");
        let n: usize = n_token.parse().unwrap_or(0);
        let mut pieces: Vec<Piece> = Vec::with_capacity(n);
        for _ in 0..n {
            let h0_tok = it.next().unwrap_or("0");
            let w0_tok = it.next().unwrap_or("0");
            let h0: usize = h0_tok.parse().unwrap_or(0);
            let w0: usize = w0_tok.parse().unwrap_or(0);
            let mut grid: Vec<Vec<char>> = Vec::with_capacity(h0);
            for _ in 0..h0 {
                let line = it.next().unwrap_or("");
                let mut row: Vec<char> = line.chars().collect();
                if row.len() < w0 {
                    row.resize(w0, '.');
                } else if row.len() > w0 {
                    row.truncate(w0);
                }
                grid.push(row);
            }
            let piece = make_piece(grid);
            pieces.push(piece);
        }

        // read p queries
        let p_token = it.next().unwrap_or("0");
        let p: usize = p_token.parse().unwrap_or(0);
        for _ in 0..p {
            let k_tok = it.next().unwrap_or("0");
            let k: usize = k_tok.parse().unwrap_or(0);
            let mut rem: Vec<usize> = Vec::with_capacity(k);
            for _ in 0..k {
                let idx_tok = it.next().unwrap_or("1");
                // piece indices are 1-based in input
                let idx1: usize = idx_tok.parse().unwrap_or(1);
                let idx0 = if idx1 == 0 { 0usize } else { idx1 - 1 };
                rem.push(idx0);
            }
            let res = solve_query(H, W, &board, &pieces, rem);
            if res {
                outputs.push("YES".to_string());
            } else {
                outputs.push("NO".to_string());
            }
        }
    }

    // print outputs
    for line in outputs {
        println!("{}", line);
    }
}
