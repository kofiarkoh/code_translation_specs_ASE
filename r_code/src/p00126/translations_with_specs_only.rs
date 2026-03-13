// Transliteration of the provided Specification into Rust.
// Behavior:
// - Read an integer `n` then `n` grids of 81 integers each from stdin.
// - For each 9x9 grid, detect duplicates in rows and columns.
// - Print each grid with a '*' prefix for flagged cells (duplicates in row or column),
//   or a single space ' ' for unflagged cells. Each printed item is "%c%d" with no
//   separators between items on a line. After each grid, print a blank line.
//
// Notes on choices to remain faithful to the spec:
// - The spec assumes cell values in 0..9 and mentions that out-of-range values
//   in the original C code would index counting arrays and cause undefined behavior.
//   To avoid panics in Rust while keeping behavior predictable, values used as
//   indices are clamped to the 0..=9 range before indexing counting arrays.
//   This prevents runtime panics while keeping the typical intended behavior for
//   valid inputs unchanged. This is the narrowest safe choice to avoid unwinding.
// - The spec mentions a buggy box-checking routine that operates on different
//   storage and therefore does not affect printed flags. We include such a
//   routine (check_boxes_buggy) that intentionally operates on a separate
//   temporary array and has no effect on the output flags.

use std::io::{self, Read};

fn clamp_index_for_count(v: i32) -> usize {
    // Clamp to [0,9] so indexing into counting arrays of length 10 cannot panic.
    // The original C code would have undefined behavior for out-of-range values;
    // this clamp is a narrow, defensive choice to keep the program from panicking
    // while preserving behavior for values in 0..=9.
    if v < 0 {
        0usize
    } else if v > 9 {
        9usize
    } else {
        v as usize
    }
}

fn check_boxes_buggy() {
    // This routine is intentionally "buggy" relative to the main checks:
    // it operates on different storage and its results are discarded, so it
    // does not affect the flags printed for the grid (as per Specification).
    //
    // We perform some computations on a separate dummy structure to emulate
    // the original program's unreachable/ineffective box-checking behavior.
    let mut dummy_grid: [[i32; 9]; 9] = [[0; 9]; 9];
    // Fill dummy grid with a pattern (here zeros) and run a counting pass
    // per 3x3 box just to emulate the presence of such a routine.
    for box_r in 0..3 {
        for box_c in 0..3 {
            let mut counts = [0i32; 10];
            for r in (box_r * 3)..(box_r * 3 + 3) {
                for c in (box_c * 3)..(box_c * 3 + 3) {
                    let v = dummy_grid[r][c];
                    let vi = clamp_index_for_count(v);
                    counts[vi] += 1;
                }
            }
            // The routine might mark something into another array in the original;
            // here we compute but intentionally discard results to preserve the bug.
            let _ = counts;
        }
    }
    // No return; side-effects intentionally absent.
}

fn main() {
    // Read all input tokens as whitespace-separated items
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we couldn't read stdin, exit quietly (matches "no return-value checks" spirit).
        return;
    }

    // Parse tokens into integers. If any token fails to parse, treat as malformed input
    // and stop processing (spec says behavior unspecified for malformed input).
    let mut nums: Vec<i32> = Vec::new();
    for tok in input.split_whitespace() {
        match tok.parse::<i32>() {
            Ok(v) => nums.push(v),
            Err(_) => {
                // Malformed token -> exit without producing further output.
                return;
            }
        }
    }

    if nums.is_empty() {
        // No tokens at all: nothing to do.
        return;
    }

    // First token is n: number of grids to process.
    let n = nums[0];
    // According to the spec, if n == 0 nothing is processed; negative n results in no iterations.
    if n <= 0 {
        return;
    }

    // Process up to n grids, each requiring 81 integers. If there are insufficient tokens,
    // stop processing further grids (the spec indicates the original didn't check scanf results).
    let mut idx = 1usize; // index into nums for the next grid cell
    for _grid_idx in 0..(n as usize) {
        if idx + 81 > nums.len() {
            // Not enough tokens remain for a full grid: stop processing.
            break;
        }

        // Read grid in row-major order into a 9x9 array
        let mut grid: [[i32; 9]; 9] = [[0; 9]; 9];
        for i in 0..81 {
            let r = i / 9;
            let c = i % 9;
            grid[r][c] = nums[idx + i];
        }
        idx += 81;

        // Flags array indicating whether to prefix '*' (true) or ' ' (false)
        let mut flags: [[bool; 9]; 9] = [[false; 9]; 9];

        // Row duplicate detection
        for r in 0..9 {
            let mut counts = [0i32; 10];
            for c in 0..9 {
                let v = grid[r][c];
                let vi = clamp_index_for_count(v);
                counts[vi] += 1;
            }
            for c in 0..9 {
                let v = grid[r][c];
                let vi = clamp_index_for_count(v);
                if counts[vi] > 1 {
                    flags[r][c] = true;
                }
            }
        }

        // Column duplicate detection
        for c in 0..9 {
            let mut counts = [0i32; 10];
            for r in 0..9 {
                let v = grid[r][c];
                let vi = clamp_index_for_count(v);
                counts[vi] += 1;
            }
            for r in 0..9 {
                let v = grid[r][c];
                let vi = clamp_index_for_count(v);
                if counts[vi] > 1 {
                    flags[r][c] = true;
                }
            }
        }

        // Call the buggy box-checking routine that operates on different storage
        // and does not affect `flags` (preserving the observable bug per Specification).
        check_boxes_buggy();

        // Print the annotated grid: 9 lines, each line has 9 items "%c%d" concatenated,
        // then a blank line after each grid.
        for r in 0..9 {
            let mut line = String::new();
            for c in 0..9 {
                let mark = if flags[r][c] { '*' } else { ' ' };
                line.push(mark);
                line.push_str(&grid[r][c].to_string());
            }
            println!("{}", line);
        }
        println!();
    }
}
