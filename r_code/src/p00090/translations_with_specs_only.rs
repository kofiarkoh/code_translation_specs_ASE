// Translated from the provided Specification into Rust.
// Behavior strictly follows the spec; where the spec was ambiguous about the
// final comparison rule for updating `max_overlap` from per-vertex min/max
// positive overlap counts, this implementation chooses the narrowest reasonable
// interpretation: update the global `max_overlap` with the maximum per-vertex
// maximum positive overlap. (The spec requires computing both min and max per
// vertex; we compute both but only use the per-vertex max to update the global
// value.)
// This choice preserves observable I/O behavior consistent with the described
// operations and keeps the implementation faithful to the stated steps.
//
// Notes:
// - Parsing accepts coordinates like "1.0,2.0", "1.0 , 2.0", or with surrounding whitespace.
// - Distance comparison uses squared distance <= 4.0 (f64).
// - Pruning removes vertices whose current degree is 1 or 2 (self-adjacency counts).
//   If any removed vertex had degree exactly 2, max_overlap is raised to at least 2.
// - After pruning, triangles are counted per anchor i: for each unordered pair (j,k)
//   with j<k, if i adjacent to j & k and j adjacent to k, then increment overlap[i][j]
//   and overlap[i][k] by 1.
// - Final printed value per test case is max_overlap + 1 (as an integer on its own line).

use std::io::{self, Read};

struct Parser {
    s: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(input: String) -> Self {
        Parser { s: input.chars().collect(), pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.s.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<char> {
        if self.pos < self.s.len() {
            let c = self.s[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    // Parse an integer (for n). Returns None on EOF or if no digits found.
    fn parse_int(&mut self) -> Option<i32> {
        self.skip_whitespace();
        let start = self.pos;
        // optional sign
        if let Some(c) = self.peek() {
            if c == '+' || c == '-' {
                self.pos += 1;
            }
        }
        let mut has_digits = false;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                has_digits = true;
                self.pos += 1;
            } else {
                break;
            }
        }
        if !has_digits {
            // no integer found; reset position to start to be conservative
            self.pos = start;
            return None;
        }
        let s: String = self.s[start..self.pos].iter().collect();
        s.parse::<i32>().ok()
    }

    // Parse a floating point number starting at current position.
    // Returns None if no number found.
    fn parse_float_token(&mut self) -> Option<f64> {
        self.skip_whitespace();
        let start = self.pos;
        let mut seen = 0;
        // Accept characters allowed in floating point literals: digits, sign (only at start),
        // decimal point, exponent markers and their signs.
        // We'll consume a maximal run of characters that could belong to a float.
        // Be careful: a '+' or '-' is only allowed at start or immediately after 'e'/'E'.
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' {
                self.pos += 1;
                seen += 1;
            } else if (c == '+' || c == '-') {
                // allow '+' or '-' if at start or previous char is 'e' or 'E'
                if self.pos == start {
                    self.pos += 1;
                    seen += 1;
                } else if let Some(prev) = self.s.get(self.pos.wrapping_sub(1)) {
                    if *prev == 'e' || *prev == 'E' {
                        self.pos += 1;
                        seen += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if seen == 0 {
            return None;
        }
        let token: String = self.s[start..self.pos].iter().collect();
        // Try parse; if parse fails, return None
        token.parse::<f64>().ok()
    }

    // Parse a coordinate pair of form: <float> , <float>
    // Allows optional whitespace around numbers and comma.
    // Returns None if parsing fails / EOF.
    fn parse_point(&mut self) -> Option<(f64, f64)> {
        self.skip_whitespace();
        // parse first float
        let x = self.parse_float_token()?;
        // skip whitespace, allow optional whitespace before comma
        self.skip_whitespace();
        // consume comma if present; the spec indicates a comma separates the two values.
        // Accept if current char is ','; if not, fail parsing to stay faithful to spec.
        if let Some(c) = self.peek() {
            if c == ',' {
                self.pos += 1;
            } else {
                // If not comma, but maybe there was no whitespace and numbers were split by whitespace,
                // try to allow that as a lenient behavior (some inputs may be "1.0 2.0"). However,
                // the spec explicitly uses "%lf,%lf" so we require a comma. We'll be slightly lenient
                // and try to skip whitespace and check again.
                // But per spec we should accept surrounding whitespace, so check again after skipping whitespace.
                // (This makes "1.0 , 2.0" accepted).
                // If still not comma, fail.
                // However, don't be overly permissive.
                // So: fail.
                return None;
            }
        } else {
            return None;
        }
        // parse second float
        let y = self.parse_float_token()?;
        Some((x, y))
    }
}

fn main() {
    // Read all stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut p = Parser::new(input);

    let mut outputs: Vec<i32> = Vec::new();

    // Process test cases until n == 0 or EOF
    loop {
        // parse next int n
        let n_opt = p.parse_int();
        let n = match n_opt {
            Some(v) => v,
            None => break, // EOF or no more integers; terminate
        };
        if n == 0 {
            break;
        }
        let n_usize = n as usize;
        // Read n points
        let mut pts: Vec<(f64, f64)> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            match p.parse_point() {
                Some(pt) => pts.push(pt),
                None => {
                    // Spec: input is well-formed; behavior on malformed input is unspecified.
                    // We'll terminate parsing gracefully.
                    return;
                }
            }
        }

        // Build adjacency matrix flag[i][j]: true if i==j or squared distance <= 4.0
        let mut flag = vec![vec![false; n_usize]; n_usize];
        for i in 0..n_usize {
            for j in 0..n_usize {
                if i == j {
                    flag[i][j] = true;
                } else {
                    let dx = pts[i].0 - pts[j].0;
                    let dy = pts[i].1 - pts[j].1;
                    let dist2 = dx * dx + dy * dy;
                    if dist2 <= 4.0_f64 {
                        flag[i][j] = true;
                    }
                }
            }
        }

        // Pruning: repeatedly remove vertices whose current degree is 1 or 2.
        // degree is sum of row (count of true flags in row).
        let mut max_overlap: i32 = 1;
        loop {
            let mut removed_in_pass = false;
            // Important: iterate i from 0..n-1; when we remove, we clear row and column immediately,
            // which affects degrees of later vertices in the same pass, matching the iterative behavior.
            for i in 0..n_usize {
                // compute degree
                let mut degree = 0;
                for j in 0..n_usize {
                    if flag[i][j] {
                        degree += 1;
                    }
                }
                if degree == 1 || degree == 2 {
                    // remove vertex i: clear its row and column
                    for j in 0..n_usize {
                        flag[i][j] = false;
                        flag[j][i] = false;
                    }
                    if degree == 2 {
                        if max_overlap < 2 {
                            max_overlap = 2;
                        }
                    }
                    removed_in_pass = true;
                    // continue scanning other vertices in this pass
                }
            }
            if !removed_in_pass {
                break;
            }
        }

        // Compute overlap counts after pruning
        let mut overlap = vec![vec![0i32; n_usize]; n_usize];
        for i in 0..n_usize {
            for j in 0..n_usize {
                // iterate unordered pairs j<k to count triangles anchored at i
                // but outer loop here is j, inner loop below will be k>j
            }
            // We'll implement below with nested loops
        }
        for i in 0..n_usize {
            for j in 0..n_usize {
                for k in (j + 1)..n_usize {
                    if flag[i][j] && flag[i][k] && flag[j][k] {
                        // triangle (i, j, k) found; increment for anchor i for both partners j and k
                        overlap[i][j] += 1;
                        overlap[i][k] += 1;
                    }
                }
            }
        }

        // For each vertex i, compute maximum and minimum positive overlap counts across partners j.
        // Then update global max_overlap according to the (ambiguous) program comparison rule.
        // Per the specification ambiguity, we update global max_overlap with the per-vertex maximum overlap.
        for i in 0..n_usize {
            let mut mx: i32 = 0;
            let mut mn_pos: Option<i32> = None;
            for j in 0..n_usize {
                let v = overlap[i][j];
                if v > 0 {
                    if v > mx {
                        mx = v;
                    }
                    mn_pos = Some(match mn_pos {
                        Some(prev) => if v < prev { v } else { prev },
                        None => v,
                    });
                }
            }
            if mx > max_overlap {
                max_overlap = mx;
            }
            // mn_pos computed but not used in the chosen interpretation.
            let _ = mn_pos;
        }

        // Output max_overlap + 1
        outputs.push(max_overlap + 1);
    }

    // Print outputs, one per line
    let mut out = String::new();
    for v in outputs {
        out.push_str(&format!("{}\n", v));
    }
    print!("{}", out);
}
