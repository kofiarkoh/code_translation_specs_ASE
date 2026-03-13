use std::cmp;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut header_line = String::new();
    // Loop over test cases
    loop {
        header_line.clear();
        // Read a non-empty line to parse h and w. If EOF, stop.
        let bytes_read = match reader.read_line(&mut header_line) {
            Ok(0) => break, // EOF
            Ok(n) => n,
            Err(_) => break,
        };
        if bytes_read == 0 {
            break;
        }

        // Skip lines that contain only whitespace, looking for header with two ints
        let mut tokens: Vec<&str> = header_line.split_whitespace().collect();
        while tokens.len() < 2 {
            header_line.clear();
            match reader.read_line(&mut header_line) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    tokens = header_line.split_whitespace().collect();
                }
                Err(_) => break,
            }
        }
        if tokens.len() < 2 {
            break;
        }

        // Parse h and w
        let h: usize = match tokens[0].parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let w: usize = match tokens[1].parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Termination condition: if h == 0 or w == 0, stop processing (do not output)
        if h == 0 || w == 0 {
            break;
        }

        // Read h rows, each expected to contain at least w characters.
        // We try to be robust: if a read line is shorter than w, keep reading and appending until we have at least w bytes.
        let mut grid_rows: Vec<Vec<u8>> = Vec::with_capacity(h);
        for _ in 0..h {
            let mut row_buf = String::new();
            // Read next non-empty line (in the original C code a leading whitespace before the grid is skipped).
            // However, grid characters can include spaces; typical inputs provide each row on its own line.
            // We'll read lines until we obtain at least one character (to avoid treating stray blank lines as rows).
            loop {
                row_buf.clear();
                match reader.read_line(&mut row_buf) {
                    Ok(0) => break, // EOF; we'll proceed with what we have (spec assumes well-formed input)
                    Ok(_) => {
                        // If the line only contains a newline (empty), skip it and continue reading.
                        if row_buf.trim_end_matches(&['\n', '\r'][..]).is_empty() {
                            // It might be a legitimately empty row (containing only newline). In the original C behavior,
                            // leading whitespace before the grid rows is skipped; empty lines would be skipped here.
                            continue;
                        } else {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }

            // Remove trailing newline/carriage return but preserve other whitespace
            while row_buf.ends_with('\n') || row_buf.ends_with('\r') {
                row_buf.pop();
            }
            let mut bytes = row_buf.into_bytes();

            // If the collected bytes are shorter than w, keep reading and appending subsequent lines until we have >= w.
            while bytes.len() < w {
                let mut extra = String::new();
                match reader.read_line(&mut extra) {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        // Remove trailing newline/carriage return and append raw bytes
                        let mut extra_bytes = extra.into_bytes();
                        while extra_bytes.ends_with(&[b'\n']) || extra_bytes.ends_with(&[b'\r']) {
                            extra_bytes.pop();
                        }
                        bytes.extend(extra_bytes);
                    }
                    Err(_) => break,
                }
            }

            // Take the first w bytes as the row (spec assumes each row has at least w characters)
            if bytes.len() >= w {
                bytes.truncate(w);
            } else {
                // If still short (malformed input), pad with non-dot characters so they are treated as obstacles.
                bytes.resize(w, b'#');
            }
            grid_rows.push(bytes);
        }

        // Build mat: for each cell mat[i][j] = number of consecutive '.' to the left ending at (i,j)
        let mut mat: Vec<Vec<usize>> = vec![vec![0usize; w]; h];
        for i in 0..h {
            let mut run = 0usize;
            for j in 0..w {
                if grid_rows[i][j] == b'.' {
                    run += 1;
                } else {
                    run = 0;
                }
                mat[i][j] = run;
            }
        }

        // Compute answer following the exact algorithm in the specification / C code:
        // For each column j as right boundary (from w-1 down to 0),
        // for each starting row k (from h-1 down to 0),
        // extend downward considering blocks of height >= 2 (i starts at 1),
        // keep minimum width across the rows in the block, break if it becomes 0.
        let mut ans: usize = 0;
        if h >= 2 {
            for j in (0..w).rev() {
                for k in (0..h).rev() {
                    let mut min_width = mat[k][j];
                    // i is the offset from starting row k; i=1 corresponds to height 2
                    let mut i = 1usize;
                    while k + i < h {
                        min_width = cmp::min(min_width, mat[k + i][j]);
                        if min_width == 0 {
                            break;
                        }
                        let area = min_width * (i + 1);
                        if area > ans {
                            ans = area;
                        }
                        i += 1;
                    }
                }
            }
        }
        // If h < 2, ans stays 0 (per the original program's behavior excluded single-row rectangles)

        println!("{}", ans);
    }
}
