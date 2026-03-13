use std::io::{self, Read};

/// A simple input scanner over raw bytes that supports:
/// - skipping ASCII whitespace
/// - reading the next integer (as isize)
/// - reading the next non-whitespace character (as u8)
///
/// This mirrors the "whitespace-skipping character read" described in the specification.
struct Scanner {
    buf: Vec<u8>,
    pos: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).expect("Failed to read stdin");
        Self { buf: s.into_bytes(), pos: 0 }
    }

    /// Advance position while at ASCII whitespace
    fn skip_ws(&mut self) {
        while self.pos < self.buf.len() && (self.buf[self.pos] as char).is_whitespace() {
            self.pos += 1;
        }
    }

    /// Read next integer (possibly zero). Assumes well-formed integer tokens per spec.
    fn next_isize(&mut self) -> isize {
        self.skip_ws();
        let start = self.pos;
        if self.pos < self.buf.len() && (self.buf[self.pos] == b'-') {
            self.pos += 1;
        }
        while self.pos < self.buf.len() && (self.buf[self.pos] as char).is_ascii_digit() {
            self.pos += 1;
        }
        let token = std::str::from_utf8(&self.buf[start..self.pos]).expect("Invalid UTF-8 in integer token");
        token.parse::<isize>().expect("Failed to parse integer")
    }

    /// Read the next non-whitespace character (single byte).
    /// Per specification, grid characters are one of '>', '<', '^', 'v', '.'
    fn next_nonws_char(&mut self) -> u8 {
        self.skip_ws();
        if self.pos >= self.buf.len() {
            panic!("Unexpected end of input while reading grid characters");
        }
        let b = self.buf[self.pos];
        self.pos += 1;
        b
    }
}

fn main() {
    let mut scanner = Scanner::new();
    let mut out_lines: Vec<String> = Vec::new();

    loop {
        let h = scanner.next_isize();
        let w = scanner.next_isize();
        // Termination condition: pair "0 0" ends processing and is not followed by grid data.
        if h == 0 && w == 0 {
            break;
        }
        // As per specification preconditions, assume 1 <= h,w <= 100 for processed cases.
        let h_usize = h as usize;
        let w_usize = w as usize;
        let total = h_usize.checked_mul(w_usize).expect("Grid size overflow");

        // Read exactly h*w non-whitespace characters into tiles in row-major order.
        let mut tiles: Vec<u8> = Vec::with_capacity(total);
        for _ in 0..total {
            let ch = scanner.next_nonws_char();
            tiles.push(ch);
        }

        // visited flags: 0 = not visited, >0 = visited (we only need boolean, but spec mentions counters)
        let mut visited: Vec<u8> = vec![0; total];

        // Current position (row, col), zero-based. Use isize so moves may produce negative values
        // which will later produce out-of-bounds indexing (matching unspecified behavior for bad inputs).
        let mut row: isize = 0;
        let mut col: isize = 0;

        // Traverse according to specification:
        // At each step:
        //  - If current cell is '.', output "col row"
        //  - Else if current cell has been visited before (visited != 0), output "LOOP"
        //  - Else mark visited and move according to arrow
        //
        // This order ensures that starting on '.' yields immediate termination without marking,
        // and revisiting a previously marked cell yields "LOOP".
        loop {
            // Compute index for current cell. This may panic if out-of-bounds (as per undefined behavior
            // described in the specification for invalid inputs). Converting negative isize to usize
            // will produce a large index and cause a bounds panic.
            let r_us = row as usize;
            let c_us = col as usize;
            let idx = r_us * w_usize + c_us;
            let cell = tiles[idx];

            if cell == b'.' {
                out_lines.push(format!("{} {}", c_us, r_us));
                break;
            }

            if visited[idx] != 0 {
                out_lines.push(String::from("LOOP"));
                break;
            }

            // Mark visited (increment once)
            visited[idx] = visited[idx].wrapping_add(1); // wrapping_add preserves single-byte semantics

            // Move according to arrow
            match cell {
                b'>' => col += 1,
                b'<' => col -= 1,
                b'^' => row -= 1,
                b'v' => row += 1,
                _ => {
                    // Per specification, behavior for unexpected characters is undefined.
                    // Here we choose to panic to reflect invalid input rather than invent behavior.
                    panic!("Invalid grid character encountered: {}", cell as char);
                }
            }
            // Loop continues; if new position is out-of-bounds, subsequent indexing will panic (undefined).
        }
    }

    // Print results: one line per test case
    let stdout = out_lines.join("\n");
    if !stdout.is_empty() {
        println!("{}", stdout);
    }
}
