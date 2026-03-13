// allpairs.rs
// Translated from the provided C program according to the given specification.
// Behavior: read n, m, then m lines "a,b,c,d", compute all-pairs shortest paths
// with Floyd–Warshall, then read one final "a,b,c,d" query and print
// c - dist(a,b) - dist(b,a) - d followed by a newline.

// Uses safe, idiomatic Rust. Parsing is strict about commas in the edge and
// query lines to match the scanf("%d,%d,%d,%d") behavior described in the spec.

use std::io::{self, Read};
use std::process::exit;

const MAX_N: usize = 20;
const INF: i32 = 1 << 21;

fn is_whitespace_byte(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C)
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            bytes: s.as_bytes(),
            pos: 0,
        }
    }

    // Skip ASCII whitespace
    fn skip_ws(&mut self) {
        while self.pos < self.bytes.len() && is_whitespace_byte(self.bytes[self.pos]) {
            self.pos += 1;
        }
    }

    // Parse an integer similar to scanf("%d"), allowing optional leading whitespace and sign.
    // Does not accept '+' as part of number in C's %d? scanf does accept '+', so we accept '+' and '-'.
    fn parse_int(&mut self) -> Result<i32, String> {
        self.skip_ws();
        if self.pos >= self.bytes.len() {
            return Err("Unexpected end of input when expecting integer".to_string());
        }
        let start = self.pos;
        // optional sign
        if self.bytes[self.pos] == b'+' || self.bytes[self.pos] == b'-' {
            self.pos += 1;
        }
        let mut any_digit = false;
        while self.pos < self.bytes.len() && (self.bytes[self.pos] as char).is_ascii_digit() {
            any_digit = true;
            self.pos += 1;
        }
        if !any_digit {
            return Err(format!(
                "Expected digits for integer at position {}",
                start
            ));
        }
        // parse substring
        let s = std::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|e| format!("Invalid UTF-8 in number: {}", e))?;
        s.parse::<i32>()
            .map_err(|e| format!("Failed to parse integer '{}': {}", s, e))
    }

    // Expect a specific character (no skipping of whitespace). This models scanf's literal behavior.
    fn expect_char(&mut self, c: u8) -> Result<(), String> {
        if self.pos >= self.bytes.len() {
            return Err(format!("Expected '{}' but reached end of input", c as char));
        }
        if self.bytes[self.pos] != c {
            return Err(format!(
                "Expected '{}' at position {}, found '{}'",
                c as char,
                self.pos,
                self.bytes[self.pos] as char
            ));
        }
        self.pos += 1;
        Ok(())
    }

    // Like parse_int but does NOT skip whitespace before the integer.
    // Used for formats where literal characters (like commas) are expected directly before numbers.
    fn parse_int_no_ws(&mut self) -> Result<i32, String> {
        // Do NOT skip whitespace here to match scanf's behavior where a literal comma
        // must immediately follow the previous token (no whitespace allowed between number and comma).
        if self.pos >= self.bytes.len() {
            return Err("Unexpected end of input when expecting integer (no ws)".to_string());
        }
        let start = self.pos;
        // optional sign
        if self.bytes[self.pos] == b'+' || self.bytes[self.pos] == b'-' {
            self.pos += 1;
        }
        let mut any_digit = false;
        while self.pos < self.bytes.len() && (self.bytes[self.pos] as char).is_ascii_digit() {
            any_digit = true;
            self.pos += 1;
        }
        if !any_digit {
            return Err(format!(
                "Expected digits for integer (no ws) at position {}",
                start
            ));
        }
        let s = std::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|e| format!("Invalid UTF-8 in number: {}", e))?;
        s.parse::<i32>()
            .map_err(|e| format!("Failed to parse integer '{}': {}", s, e))
    }

    // Parse a quartet of integers separated by literal commas, with optional leading whitespace
    // before the first integer (mimics scanf("%d,%d,%d,%d")).
    fn parse_csv4(&mut self) -> Result<(i32, i32, i32, i32), String> {
        // scanf("%d,%d,%d,%d") will skip whitespace before the first %d, then require comma immediately after each integer.
        self.skip_ws();
        let a = self.parse_int_no_ws()?;
        self.expect_char(b',')?;
        let b = self.parse_int_no_ws()?;
        self.expect_char(b',')?;
        let c = self.parse_int_no_ws()?;
        self.expect_char(b',')?;
        let d = self.parse_int_no_ws()?;
        Ok((a, b, c, d))
    }
}

fn main() {
    // Read whole stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, mimic undefined behavior by exiting non-zero.
        exit(1);
    }

    let mut p = Parser::new(&input);

    // First read n and m using whitespace-separated integers (scanf("%d%d")).
    // We'll use parse_int which skips whitespace before numbers.
    let n = match p.parse_int() {
        Ok(v) => v,
        Err(_) => {
            // Undefined input -> exit non-zero
            exit(1);
        }
    };
    let m = match p.parse_int() {
        Ok(v) => v,
        Err(_) => exit(1),
    };

    if n < 1 || n as usize > MAX_N {
        // Spec says behavior undefined if out of range; choose to exit non-zero.
        // (Could panic, but exit to keep output empty as undefined).
        exit(1);
    }

    let n_usize = n as usize;

    // Initialize matrix
    let mut mat = vec![vec![INF; n_usize]; n_usize];
    for i in 0..n_usize {
        mat[i][i] = 0;
    }

    // Read m edges, each in format "%d,%d,%d,%d" with literal commas.
    for _ in 0..m {
        let (aa, bb, cc, dd) = match p.parse_csv4() {
            Ok(t) => t,
            Err(_) => exit(1),
        };
        // convert to zero-based indices
        let a_idx = (aa - 1) as isize;
        let b_idx = (bb - 1) as isize;
        if a_idx < 0 || b_idx < 0 || a_idx as usize >= n_usize || b_idx as usize >= n_usize {
            // Out-of-range -> undefined; exit non-zero.
            exit(1);
        }
        let a_us = a_idx as usize;
        let b_us = b_idx as usize;
        mat[a_us][b_us] = cc;
        mat[b_us][a_us] = dd;
    }

    // Floyd–Warshall: iterate intermediate i, then j, then k, matching the C ordering.
    for i in 0..n_usize {
        for j in 0..n_usize {
            for k in 0..n_usize {
                // Be mindful of potential overflow in addition; spec guarantees sums < INF.
                let via = mat[j][i].saturating_add(mat[i][k]);
                if via < mat[j][k] {
                    mat[j][k] = via;
                }
            }
        }
    }

    // Read final query in "%d,%d,%d,%d"
    let (aa, bb, cc, dd) = match p.parse_csv4() {
        Ok(t) => t,
        Err(_) => exit(1),
    };
    let a_idx = (aa - 1) as isize;
    let b_idx = (bb - 1) as isize;
    if a_idx < 0 || b_idx < 0 || a_idx as usize >= n_usize || b_idx as usize >= n_usize {
        exit(1);
    }
    let a_us = a_idx as usize;
    let b_us = b_idx as usize;

    // Compute result: c - dist(a,b) - dist(b,a) - d
    let result = cc - mat[a_us][b_us] - mat[b_us][a_us] - dd;

    // Print exactly one integer followed by newline.
    println!("{}", result);
}
