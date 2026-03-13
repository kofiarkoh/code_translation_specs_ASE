// sliding_2x4_bfs.rs
//
// BFS solver for the 2x4 sliding tile puzzle with tiles labeled 0..7.
// The blank tile is represented by '0'. For each input line, the program
// reads the first eight digit characters (row-major order) and prints the
// minimal number of moves to reach the goal state [0,1,2,3,4,5,6,7].
//
// This implementation follows the provided specification as the sole source
// of truth. It maps permutations of 0..7 to a rank in [0..8!-1] using the
// factorial-number (Lehmer) code to index a visited table of fixed size.
// BFS explores neighbors by swapping the blank with adjacent cells in a
// 2x4 grid. If the input line does not contain at least eight digit
// characters in the expected positions, behavior is unspecified by the
// specification; this program will skip such lines. If a parsed line does
// not form a valid permutation of 0..7, this program will print -1.
//
// Note: printing -1 for invalid/unsolvable cases is a minor pragmatic
// choice to produce defined output rather than leaving behavior entirely
// unspecified. The specification assumes valid, solvable inputs.

use std::io::{self, BufRead};
use std::collections::VecDeque;

const FACTORIALS: [usize; 9] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320
];
const PERM_COUNT: usize = 40320; // 8!

fn perm_rank(perm: &[u8; 8]) -> usize {
    // Lehmer code / factorial number system rank for permutations of 0..7
    // Assumes perm contains a permutation of 0..7.
    let mut rank = 0usize;
    let mut used = [false; 8];
    for i in 0..8 {
        let v = perm[i] as usize;
        let mut smaller_unused = 0usize;
        for x in 0..v {
            if !used[x] {
                smaller_unused += 1;
            }
        }
        // number of permutations skipped at this position
        rank += smaller_unused * FACTORIALS[7 - i];
        used[v] = true;
    }
    rank
}

fn parse_line_to_perm(line: &str) -> Option<[u8; 8]> {
    // Following the specification, the program expects eight digit characters
    // encoding tiles 0..7 in row-major order. We'll collect the first eight
    // characters that are digits '0'..'7'. If less than eight such digits
    // are present, return None (unspecified behavior per spec).
    let mut perm = [0u8; 8];
    let mut idx = 0usize;
    for b in line.bytes() {
        if idx >= 8 {
            break;
        }
        if b'0' <= b && b <= b'7' {
            perm[idx] = (b - b'0') as u8;
            idx += 1;
        }
    }
    if idx < 8 {
        return None;
    }
    // Optionally validate permutation (each 0..7 appears exactly once).
    // The spec says behavior is unspecified on invalid inputs, but we
    // return None in that case to avoid UB.
    let mut seen = [false; 8];
    for &v in &perm {
        let vi = v as usize;
        if vi >= 8 || seen[vi] {
            return None;
        }
        seen[vi] = true;
    }
    Some(perm)
}

fn find_zero(possible: &[u8; 8]) -> usize {
    for i in 0..8 {
        if possible[i] == 0 {
            return i;
        }
    }
    0
}

fn bfs_min_moves(start: [u8; 8]) -> Option<usize> {
    // Goal is [0,1,2,3,4,5,6,7]
    const GOAL: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    if start == GOAL {
        return Some(0);
    }

    // visited table indexed by permutation rank
    let mut visited = vec![false; PERM_COUNT];
    let start_rank = perm_rank(&start);
    visited[start_rank] = true;

    // BFS queue entries: (perm, zero_pos, depth)
    let mut q: VecDeque<([u8;8], usize, usize)> = VecDeque::new();
    let start_zero = find_zero(&start);
    q.push_back((start, start_zero, 0usize));

    while let Some((perm, zero_pos, depth)) = q.pop_front() {
        let zr = zero_pos;
        let zr_r = zr / 4;
        let zr_c = zr % 4;

        // four cardinal directions: up, down, left, right
        let deltas = [(-1isize, 0isize), (1,0), (0,-1), (0,1)];
        for &(dr, dc) in &deltas {
            let nr = (zr_r as isize) + dr;
            let nc = (zr_c as isize) + dc;
            if nr < 0 || nr > 1 || nc < 0 || nc > 3 {
                continue;
            }
            let npos = (nr as usize) * 4 + (nc as usize);
            let mut next = perm;
            next[zr] = next[npos];
            next[npos] = 0u8;

            if next == GOAL {
                return Some(depth + 1);
            }

            let r = perm_rank(&next);
            if !visited[r] {
                visited[r] = true;
                q.push_back((next, npos, depth + 1));
            }
        }
    }

    // If BFS exhausts queue without finding goal, behavior is unspecified
    // by the provided specification. Return None to indicate "not found".
    None
}

fn main() {
    let stdin = io::stdin();
    for line_res in stdin.lock().lines() {
        match line_res {
            Ok(line) => {
                match parse_line_to_perm(&line) {
                    Some(start_perm) => {
                        match bfs_min_moves(start_perm) {
                            Some(moves) => {
                                println!("{}", moves);
                            }
                            None => {
                                // Unspecified by spec; print -1 to indicate failure
                                // to find goal (for malformed or unreachable cases).
                                println!("-1");
                            }
                        }
                    }
                    None => {
                        // Input did not contain 8 valid digits 0..7 in expected positions
                        // (spec leaves behavior unspecified). We choose to print -1.
                        println!("-1");
                    }
                }
            }
            Err(_) => {
                // On IO error reading lines, stop processing.
                break;
            }
        }
    }
}
