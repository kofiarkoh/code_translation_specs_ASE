// translate_spec.rs
//
// Translates the provided specification into Rust.
// Behavior notes (kept faithful to the specification):
// - Reads multiple test cases: each begins with two integers h and w.
// - Terminates when h == 0 or w == 0 (either zero).
// - For each test case reads h subsequent lines (each line is treated as a row).
// - Treats '.' (ASCII 46) as open, any other byte as blocked.
// - Computes left-run lengths of consecutive '.' ending at each column.
// - For each column as a right boundary, for each starting row, extends downward,
//   keeping the minimum left-run for that column across the vertical block.
// - Only considers rectangles with height >= 2 (height == 1 is ignored).
// - For each such vertical block computes area = min_width * height and tracks the maximum.
// - Prints one integer result per test case on its own line.
//
// Input parsing:
// - This implementation parses the entire stdin as bytes. It parses the two integers
//   h and w by skipping whitespace and reading digits. After reading the second integer,
//   it advances to the next newline and then reads the following h lines as raw rows
//   (preserving spaces inside rows).
//
// Edge handling choices (kept conservative / safe):
// - If a row has fewer than w bytes (contrary to the spec), missing positions are
//   treated as blocked (not '.'). The spec says behavior is undefined in malformed inputs;
//   this choice avoids panics while remaining a narrow interpretation.
//
// This implementation is safe Rust and preserves the observable behavior required.

use std::io::{self, Read};

fn parse_int(buf: &[u8], idx: &mut usize) -> Option<i64> {
    let len = buf.len();
    // Skip whitespace
    while *idx < len && buf[*idx].is_ascii_whitespace() {
        *idx += 1;
    }
    if *idx >= len {
        return None;
    }
    // Optional sign
    let mut sign: i64 = 1;
    if buf[*idx] == b'-' {
        sign = -1;
        *idx += 1;
    } else if buf[*idx] == b'+' {
        *idx += 1;
    }

    // At least one digit required
    if *idx >= len || !buf[*idx].is_ascii_digit() {
        return None;
    }
    let mut num: i64 = 0;
    while *idx < len && buf[*idx].is_ascii_digit() {
        // digit
        num = num * 10 + (buf[*idx] - b'0') as i64;
        *idx += 1;
    }
    Some(sign * num)
}

fn main() {
    // Read all stdin
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // On read error, exit (no output). Spec: termination on IO failure is allowed.
        return;
    }

    let mut idx: usize = 0;
    let len = input.len();

    let mut out_lines: Vec<String> = Vec::new();

    loop {
        // Parse h
        let h_i64 = match parse_int(&input, &mut idx) {
            Some(v) => v,
            None => break, // EOF or no more integers: terminate
        };
        let w_i64 = match parse_int(&input, &mut idx) {
            Some(v) => v,
            None => break,
        };
        // Convert to usize; negative values are unlikely per spec, but if negative treat as termination
        if h_i64 <= 0 || w_i64 <= 0 {
            // Per spec: if h == 0 or w == 0 (either zero) terminate processing.
            // If negative, we treat as termination as a narrow, safe interpretation.
            break;
        }
        let h = h_i64 as usize;
        let w = w_i64 as usize;

        // Advance idx to the next newline after the second integer (so that reading rows starts at the next line).
        while idx < len && input[idx] != b'\n' {
            idx += 1;
        }
        if idx < len && input[idx] == b'\n' {
            idx += 1;
        } else {
            // No newline found (EOF). According to spec, behavior is undefined;
            // we stop processing to avoid reading past EOF.
            break;
        }

        // Read h raw lines (each up to the next newline). If a line is shorter than w,
        // missing positions are treated as blocked.
        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
        for _ in 0..h {
            if idx >= len {
                // EOF before expected number of rows; stop processing further test cases.
                // This is a narrow handling to avoid panics; spec allowed undefined behavior.
                break;
            }
            let start = idx;
            while idx < len && input[idx] != b'\n' {
                idx += 1;
            }
            let mut end = idx;
            // Strip trailing '\r' if present (handle CRLF)
            if end > start && input[end - 1] == b'\r' {
                end -= 1;
            }
            // Extract row bytes
            let row: Vec<u8> = input[start..end].to_vec();
            grid.push(row);
            // Skip the newline if present
            if idx < len && input[idx] == b'\n' {
                idx += 1;
            }
        }
        if grid.len() != h {
            // Incomplete grid; stop processing as per safe choice.
            break;
        }

        // Build left-run matrix: left_run[i][j] = number of consecutive '.' ending at column j in row i.
        // Stored as usize.
        let mut left_run: Vec<Vec<usize>> = vec![vec![0usize; w]; h];
        for i in 0..h {
            for j in 0..w {
                let ch = if j < grid[i].len() {
                    grid[i][j]
                } else {
                    // Missing characters are treated as blocked (not '.')
                    b'#'
                };
                if ch == b'.' {
                    if j == 0 {
                        left_run[i][j] = 1;
                    } else {
                        left_run[i][j] = left_run[i][j - 1] + 1;
                    }
                } else {
                    left_run[i][j] = 0;
                }
            }
        }

        // Core computation per specification:
        // For each column as right boundary, for each starting row, extend downward,
        // keeping minimum width across rows in the block. Only consider blocks with height >= 2.
        let mut ans: usize = 0;
        for col in 0..w {
            for start_row in 0..h {
                let mut min_width = usize::MAX;
                for end_row in start_row..h {
                    let width_here = left_run[end_row][col];
                    if width_here < min_width {
                        min_width = width_here;
                    }
                    let height = end_row - start_row + 1;
                    if height >= 2 {
                        let area = min_width * height;
                        if area > ans {
                            ans = area;
                        }
                    }
                    // Note: if min_width becomes 0, further extension in this column will not produce positive area,
                    // but we continue to match the described triple-loop behavior.
                }
            }
        }

        out_lines.push(format!("{}", ans));
    }

    // Print results: one integer per processed test case on its own line.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for line in out_lines {
        // Each line followed by newline
        if let Err(_) = writeln!(handle, "{}", line) {
            // If printing fails, just exit; spec says process exit code 0 on normal termination.
            return;
        }
    }
}
