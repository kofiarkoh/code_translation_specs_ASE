use std::io::{self, Read};

fn number(s: &[u8], pos: &mut usize) -> i32 {
    let mut val: i32 = 0;
    while s[*pos] >= b'0' && s[*pos] <= b'9' {
        let digit = (s[*pos] - b'0') as i32;
        // Use wrapping operations to emulate typical C (32-bit) signed overflow behavior.
        val = val.wrapping_mul(10).wrapping_add(digit);
        *pos += 1;
    }
    val
}

fn factor(s: &[u8], pos: &mut usize) -> i32 {
    if s[*pos] == b'(' {
        *pos += 1;
        let val = expr(s, pos);
        // In the C code the closing ')' is consumed unconditionally.
        *pos += 1;
        return val;
    }
    number(s, pos)
}

fn c_div(a: i32, b: i32) -> i32 {
    // Division by zero in the original C is undefined; here we let Rust panic if it occurs.
    // Emulate typical two's-complement behavior for INT_MIN / -1 by returning INT_MIN.
    if b == 0 {
        panic!("division by zero");
    }
    if a == i32::MIN && b == -1 {
        i32::MIN
    } else {
        // Rust's '/' for i32 matches C99 semantics (truncates toward zero).
        a / b
    }
}

fn term(s: &[u8], pos: &mut usize) -> i32 {
    let mut val = factor(s, pos);
    loop {
        match s[*pos] {
            b'*' => {
                *pos += 1;
                val = val.wrapping_mul(factor(s, pos));
            }
            b'/' => {
                *pos += 1;
                let rhs = factor(s, pos);
                val = c_div(val, rhs);
            }
            _ => break,
        }
    }
    val
}

fn expr(s: &[u8], pos: &mut usize) -> i32 {
    let mut val = term(s, pos);
    loop {
        match s[*pos] {
            b'+' => {
                *pos += 1;
                val = val.wrapping_add(term(s, pos));
            }
            b'-' => {
                *pos += 1;
                val = val.wrapping_sub(term(s, pos));
            }
            _ => break,
        }
    }
    val
}

fn main() {
    // Read entire stdin and split by whitespace to emulate scanf("%d", &n); scanf("%s", s);
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read failed");
    let mut tokens = input.split_whitespace();

    // Parse n
    let n_token = tokens.next().unwrap_or("0");
    let n: usize = n_token.parse().unwrap_or(0);

    for _ in 0..n {
        // Emulate scanf("%s", s) into char s[101]:
        // - take next whitespace-separated token
        // - truncate to at most 100 characters (space for null terminator in C)
        let tok = tokens.next().unwrap_or("");
        let truncated: String = tok.chars().take(100).collect();

        // Work with bytes and append a sentinel 0 byte to emulate C's '\0' terminator.
        let mut bytes = truncated.into_bytes();
        bytes.push(0);

        let mut pos: usize = 0;
        let val = expr(&bytes, &mut pos);
        println!("{}", val);
    }
}
