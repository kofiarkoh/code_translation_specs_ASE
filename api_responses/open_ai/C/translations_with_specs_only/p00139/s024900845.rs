use std::io::{self, Read};

fn is_pattern_a(s: &str) -> bool {
    // Pattern A: ^>'(=*)#\1~$
    // Using byte-level checks to match the C/ASCII semantics precisely.
    let b = s.as_bytes();
    // Minimal length is 4: '>' '\'' '#' '~' (k = 0)
    if b.len() < 4 {
        return false;
    }
    if b[0] != b'>' || b[1] != b'\'' {
        return false;
    }
    // count k '=' after the initial ">'"
    let mut i = 2usize;
    while i < b.len() && b[i] == b'=' {
        i += 1;
    }
    let k = i - 2;
    // next must be '#'
    if i >= b.len() || b[i] != b'#' {
        return false;
    }
    i += 1;
    // next must be exactly k '=' characters
    for _ in 0..k {
        if i >= b.len() || b[i] != b'=' {
            return false;
        }
        i += 1;
    }
    // next must be '~' and then end of string
    if i >= b.len() || b[i] != b'~' {
        return false;
    }
    i += 1;
    i == b.len()
}

fn is_pattern_b(s: &str) -> bool {
    // Pattern B: ^>\^(?:Q=)+~~$
    // Byte-level checks for ASCII semantics.
    let b = s.as_bytes();
    // Minimal length is 6: '>' '^' "Q=" '~~' => 2 + 2 + 2 = 6
    if b.len() < 6 {
        return false;
    }
    if b[0] != b'>' || b[1] != b'^' {
        return false;
    }
    let mut i = 2usize;
    let mut pairs = 0usize;
    // consume one-or-more "Q=" pairs
    while i + 1 < b.len() && b[i] == b'Q' && b[i + 1] == b'=' {
        pairs += 1;
        i += 2;
    }
    if pairs < 1 {
        return false;
    }
    // must have exactly two '~' and then end of string
    if i + 2 != b.len() {
        return false;
    }
    b[i] == b'~' && b[i + 1] == b'~'
}

fn main() {
    // Read entire stdin and split on whitespace to emulate scanf("%s") semantics.
    let mut input = String::new();
    // If reading stdin fails, exit silently (matches narrow interpretation of unspecified scanf failure behavior).
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut toks = input.split_whitespace();

    // Read n (first token). If missing or not an integer, behave like unspecified scanf failure:
    // choose to exit without producing outputs.
    let n_token = match toks.next() {
        Some(t) => t,
        None => return,
    };
    let n: i32 = match n_token.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    // If n <= 0, do zero iterations (no outputs), per specification.
    if n <= 0 {
        return;
    }

    // Process up to n tokens. If fewer tokens are available, stop early (scanf behavior on EOF is unspecified;
    // this is a narrow, safe interpretation).
    for _ in 0..n {
        let s = match toks.next() {
            Some(t) => t,
            None => break,
        };
        if is_pattern_a(s) {
            println!("A");
        } else if is_pattern_b(s) {
            println!("B");
        } else {
            println!("NA");
        }
    }
}
