// Translated from given C code.
// Preserves behavior of the original program for well-formed inputs.
// Notes:
// - Uses wrapping_add for data increments to emulate typical two's-complement wraparound.
// - Adds a bounds check to avoid panic on out-of-bounds indexing. The original C code would have undefined behavior in that case.
// - Reads characters by skipping whitespace and taking the next byte (matches " %c" in scanf).

use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    pos: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), pos: 0 }
    }

    // Skip ASCII whitespace
    fn skip_ws(&mut self) {
        while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    // Parse next i32, similar to scanf("%d")
    fn next_i32(&mut self) -> Option<i32> {
        self.skip_ws();
        if self.pos >= self.buf.len() {
            return None;
        }
        let mut sign = 1;
        if self.buf[self.pos] == b'+' {
            self.pos += 1;
        } else if self.buf[self.pos] == b'-' {
            sign = -1;
            self.pos += 1;
        }
        if self.pos >= self.buf.len() || !self.buf[self.pos].is_ascii_digit() {
            // No digits after optional sign; mimic failure to parse
            return None;
        }
        let mut val: i32 = 0;
        while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_digit() {
            // safe to parse as i32; matches typical scanf behavior for normal inputs
            val = val.saturating_mul(10).saturating_add((self.buf[self.pos] - b'0') as i32);
            self.pos += 1;
        }
        Some(sign * val)
    }

    // Read next non-whitespace character, similar to scanf(" %c", &c)
    fn next_non_ws_char(&mut self) -> Option<char> {
        self.skip_ws();
        if self.pos >= self.buf.len() {
            return None;
        }
        let b = self.buf[self.pos];
        self.pos += 1;
        Some(b as char)
    }
}

fn mag_tile(mut y: isize, mut x: isize, data: &mut Vec<Vec<i32>>, tiles: &Vec<Vec<char>>) {
    let h = data.len() as isize;
    let w = if h > 0 { data[0].len() as isize } else { 0 };

    // Bounds check to avoid panics. The original C would have undefined behavior if out-of-bounds.
    if y < 0 || x < 0 || y >= h || x >= w {
        return;
    }
    let yi = y as usize;
    let xi = x as usize;

    if data[yi][xi] == 1 {
        println!("LOOP");
        return;
    }
    if tiles[yi][xi] == '.' {
        // Print x y (note order: x then y)
        println!("{} {}", xi, yi);
        return;
    }

    match tiles[yi][xi] {
        '>' => {
            // data[y][x]++ then x++
            data[yi][xi] = data[yi][xi].wrapping_add(1);
            x = x + 1;
        }
        '<' => {
            // data[y][x]++ then x--
            data[yi][xi] = data[yi][xi].wrapping_add(1);
            x = x - 1;
        }
        '^' => {
            // data[y][x]++ then y--
            data[yi][xi] = data[yi][xi].wrapping_add(1);
            y = y - 1;
        }
        'v' => {
            // data[y][x]++ then y++
            data[yi][xi] = data[yi][xi].wrapping_add(1);
            y = y + 1;
        }
        _ => {
            // If an unexpected character appears, mimic C behavior: do nothing further.
            return;
        }
    }

    mag_tile(y, x, data, tiles);
}

fn main() {
    let mut scanner = Scanner::new();

    loop {
        let h = match scanner.next_i32() {
            Some(v) => v,
            None => break, // EOF or parse failure
        };
        let w = match scanner.next_i32() {
            Some(v) => v,
            None => break,
        };

        if h == 0 && w == 0 {
            break;
        }
        // Assume h,w are non-negative and reasonably small as in original problem constraints.
        let hu = h as usize;
        let wu = w as usize;

        let mut tiles: Vec<Vec<char>> = vec![vec!['.'; wu]; hu];
        let mut data: Vec<Vec<i32>> = vec![vec![0; wu]; hu];

        for i in 0..hu {
            for j in 0..wu {
                // scanf(" %c",&Tiles[i][j]) reads next non-whitespace character
                if let Some(c) = scanner.next_non_ws_char() {
                    tiles[i][j] = c;
                    data[i][j] = 0;
                } else {
                    // Unexpected EOF; mimic ending
                    return;
                }
            }
        }

        mag_tile(0, 0, &mut data, &tiles);
    }
}
