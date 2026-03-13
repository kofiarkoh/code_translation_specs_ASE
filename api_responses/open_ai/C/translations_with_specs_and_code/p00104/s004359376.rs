// translate_magtiles.rs
//
// Translation of the given C program to Rust following the provided specification.
// Behavior: For each test case, start at (0,0) and follow arrows ('>','<','^','v')
// until reaching '.' (print "col row") or revisiting a cell (print "LOOP").
// Input terminates when "0 0" is read for h and w.
//
// Notes:
// - The parser reads integers by skipping whitespace and parsing digits,
//   and reads grid cells as single non-whitespace characters (matching scanf(" %c") behavior).
// - This implementation assumes inputs satisfy the specification (enough characters,
//   valid ranges, moves remain in-bounds). For invalid inputs behavior is undefined
//   per the spec; here unwraps/panics may occur in such cases.

use std::io::{self, Read};

struct Input {
    bytes: Vec<u8>,
    pos: usize,
}

impl Input {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).expect("Failed to read stdin");
        Self {
            bytes: s.into_bytes(),
            pos: 0,
        }
    }

    // Advance pos while at whitespace.
    fn skip_ws(&mut self) {
        while let Some(&b) = self.bytes.get(self.pos) {
            if (b as char).is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    // Read next integer (skipping leading whitespace). Returns Some(i32) or None at EOF/malformed.
    fn read_i32(&mut self) -> Option<i32> {
        self.skip_ws();
        if self.pos >= self.bytes.len() {
            return None;
        }
        let mut neg = false;
        if self.bytes[self.pos] == b'-' {
            neg = true;
            self.pos += 1;
        }
        let mut any = false;
        let mut val: i32 = 0;
        while let Some(&b) = self.bytes.get(self.pos) {
            if b >= b'0' && b <= b'9' {
                any = true;
                val = val
                    .checked_mul(10)?
                    .checked_add((b - b'0') as i32)?;
                self.pos += 1;
            } else {
                break;
            }
        }
        if !any {
            None
        } else if neg {
            Some(-val)
        } else {
            Some(val)
        }
    }

    // Read a single non-whitespace character (matching " %c" semantics).
    fn read_non_ws_char(&mut self) -> Option<char> {
        self.skip_ws();
        if self.pos >= self.bytes.len() {
            return None;
        }
        let b = self.bytes[self.pos];
        self.pos += 1;
        Some(b as char)
    }
}

fn main() {
    let mut input = Input::new();

    loop {
        let h = match input.read_i32() {
            Some(v) => v,
            None => break, // EOF or malformed; end processing
        };
        let w = match input.read_i32() {
            Some(v) => v,
            None => break,
        };

        if h == 0 && w == 0 {
            break;
        }

        let h_usize = h as usize;
        let w_usize = w as usize;

        // Read grid: h * w non-whitespace characters, row-major (Tiles[row][col])
        let mut tiles: Vec<Vec<char>> = vec![vec!['.'; w_usize]; h_usize];
        let mut data: Vec<Vec<u8>> = vec![vec![0; w_usize]; h_usize];

        for i in 0..h_usize {
            for j in 0..w_usize {
                // Per spec, exactly h*w non-whitespace characters are provided.
                tiles[i][j] = input.read_non_ws_char().expect("Expected grid character");
                data[i][j] = 0;
            }
        }

        // Iterative traversal to avoid recursion (preserves semantics: mark before moving).
        let mut y: usize = 0;
        let mut x: usize = 0;

        loop {
            // Invariants per spec: 0 <= y < h, 0 <= x < w for well-formed inputs.
            if data[y][x] == 1 {
                println!("LOOP");
                break;
            }
            if tiles[y][x] == '.' {
                println!("{} {}", x, y);
                break;
            }

            // Mark visit (increment; only the fact of >0 matters)
            data[y][x] = data[y][x].saturating_add(1);

            // Move according to tile. For well-formed inputs, moves stay in bounds.
            match tiles[y][x] {
                '>' => {
                    x = x + 1;
                }
                '<' => {
                    x = x - 1;
                }
                '^' => {
                    y = y - 1;
                }
                'v' => {
                    y = y + 1;
                }
                // For characters outside the spec the original program's behavior is undefined.
                // Here we break to avoid panic; this path should not occur for valid inputs.
                _ => {
                    println!("LOOP");
                    break;
                }
            }
        }
    }
}
