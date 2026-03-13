// puzzle_solver.rs
//
// Translated from the provided C implementation into safe, idiomatic Rust.
// The SPEC is treated as authoritative. A couple of deviations from the original C
// were intentionally made to conform to the SPEC (see comments below).
//
// Behavior: For each input line, expects the eight tile digits at indices
// j*2 + i*8 for i in 0..2 (rows) and j in 0..4 (cols). Prints the minimal
// number of moves to reach the goal configuration 0,1,2,3,4,5,6,7 (one line per input).

use std::collections::VecDeque;
use std::io::{self, BufRead};

const VIS_SIZE: usize = 76_543_211; // one larger than 76,543,210 to allow indexing up to 76543210
const GOAL_HASH: usize = 1_234_567; // decimal 01234567 == 1234567

#[derive(Clone)]
struct Puzzle {
    num: [[u8; 4]; 2],
    time: i32,
    hash: usize,
    x: usize,
    y: usize,
}

fn generate_hash(num: &[[u8; 4]; 2]) -> usize {
    let mut ret: usize = 0;
    for i in 0..8 {
        ret = ret * 10 + (num[i / 4][i % 4] as usize);
    }
    ret
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Movement vectors: right, down, left, up (same order as C)
    let dx = [1isize, 0, -1, 0];
    let dy = [0isize, 1, 0, -1];

    // Read input lines until EOF
    while let Some(Ok(line)) = lines.next() {
        // The C program used "%[^\n]" then getchar() to consume newline.
        // Here `line` is the content up to but not including the newline.
        // Parse digits at positions j*2 + i*8 for i=0..1, j=0..3.
        // The SPEC requires these positions to be valid; behavior is unspecified otherwise.
        let bytes = line.as_bytes();

        let mut first = Puzzle {
            num: [[0u8; 4]; 2],
            time: 0,
            hash: 0,
            x: 0,
            y: 0,
        };

        for i in 0..2 {
            for j in 0..4 {
                let idx = j * 2 + i * 8;
                // Assume valid input as per SPEC; this will panic if input is not as expected.
                let digit = (bytes[idx] - b'0') as u8;
                first.num[i][j] = digit;
                if digit == 0 {
                    first.x = j;
                    first.y = i;
                }
            }
        }

        // Visited table: zeroed for each puzzle instance.
        // NOTE: The original C code used an array of size 76,543,210 and did not mark
        // the initial state as visited when enqueuing it. That contradicts the SPEC's
        // invariant that a visited-state marker prevents enqueuing a configuration
        // more than once. To follow the SPEC, we:
        //  - allocate a visited array large enough to index hash==76543210 (hence +1),
        //  - mark the initial state's hash as visited when enqueuing it.
        //
        // This preserves the SPEC semantics even though it differs from the original C
        // implementation which contained these likely bugs.
        let mut visited = vec![0u8; VIS_SIZE];

        first.time = 0;
        first.hash = generate_hash(&first.num);

        let mut queue: VecDeque<Puzzle> = VecDeque::new();
        // Mark initial as visited per SPEC and enqueue it.
        if first.hash < visited.len() {
            visited[first.hash] = 1;
        }
        queue.push_back(first);

        // BFS
        let result_time = loop {
            let temp = match queue.pop_front() {
                Some(p) => p,
                None => {
                    // According to SPEC, behavior when goal is unreachable is unspecified.
                    // Here we break to avoid infinite loop; print the last seen time (or 0).
                    // This branch normally shouldn't happen for valid puzzles within capacity.
                    break 0;
                }
            };

            if temp.hash == GOAL_HASH {
                break temp.time;
            }

            for k in 0..4 {
                let nx = temp.x as isize + dx[k];
                let ny = temp.y as isize + dy[k];
                if nx >= 0 && nx < 4 && ny >= 0 && ny < 2 {
                    let mut next = temp.clone();
                    next.time = temp.time + 1;
                    next.x = nx as usize;
                    next.y = ny as usize;

                    // swap the blank (at temp.y,temp.x) with tile at (next.y,next.x)
                    next.num[temp.y][temp.x].swap(&mut next.num[next.y][next.x]);

                    next.hash = generate_hash(&next.num);

                    // Follow SPEC: prevent enqueuing a state more than once using visited table.
                    // If a hash is out-of-bounds, this will panic (mirroring unspecified behavior
                    // in the original C program when capacities are exceeded).
                    if next.hash < visited.len() && visited[next.hash] == 0 {
                        visited[next.hash] = 1;
                        queue.push_back(next);
                    }
                }
            }
        };

        println!("{}", result_time);
    }
}
