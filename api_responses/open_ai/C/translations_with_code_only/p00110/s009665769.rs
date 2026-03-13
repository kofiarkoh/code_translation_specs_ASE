// translated from the provided C program
// Preserves semantics of per-iteration double arithmetic and truncation back to int
use std::io::{self, Read};

fn solved(num: &str, i: i32) -> i32 {
    // Emulate the C behavior where:
    // - pow(10, ...) is computed as a double
    // - each addition is done in floating point (promoting the current int s to double),
    //   then the result is converted back to int and stored in s (truncation toward zero).
    //
    // To mimic that precisely, we keep s as i32 and for each digit compute:
    //   temp_f64 = (s as f64) + (digit_value as f64) * pow(10, exponent)
    //   s = temp_f64 as i32; // truncates toward zero in Rust as in C for typical ranges
    //
    // We iterate over bytes to match C's char[] indexing and ASCII digit arithmetic.
    let bytes = num.as_bytes();
    let len = bytes.len();
    let mut s: i32 = 0;
    for j in 0..len {
        // exponent: strlen(num) - 1 - j
        let exp = (len - 1 - j) as f64;
        let place = 10f64.powf(exp);
        let term_f64 = if bytes[j] == b'X' {
            (i as f64) * place
        } else {
            // emulate (num[j] - '0') as in C; allow negative if non-digit (matching C behavior)
            let digit_val = (bytes[j] as i32) - (b'0' as i32);
            (digit_val as f64) * place
        };
        let tmp = (s as f64) + term_f64;
        s = tmp as i32; // truncates toward zero for normal finite values
    }
    s
}

fn main() {
    // Read all input and split on whitespace, matching scanf("%s", ...) in C
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for token in input.split_whitespace() {
        // token corresponds to calc in C, e.g. "12+3=15," or "1X+2=3"
        // We need to extract f, s, ans following the C strtok sequence:
        //   f = strtok(calc, "+");
        //   s = strtok(NULL, "=");
        //   ans = strtok(NULL, ",");
        // That behavior is effectively: split at '+' to get f and the rest;
        // then split the rest at '=' to get s and the rest;
        // then split the rest at ',' to get ans (if no comma, take the rest).
        let plus_pos = match token.find('+') {
            Some(p) => p,
            None => continue, // malformed input; C's behavior would be undefined; skip instead
        };
        let f = &token[..plus_pos];
        let rest_after_plus = &token[plus_pos + 1..];

        let eq_pos = match rest_after_plus.find('=') {
            Some(p) => p,
            None => continue,
        };
        let s_part = &rest_after_plus[..eq_pos];
        let mut ans_part = &rest_after_plus[eq_pos + 1..];

        // strtok(NULL, ",") would stop at a comma if present. If not present, it returns the rest.
        if let Some(comma_pos) = ans_part.find(',') {
            ans_part = &ans_part[..comma_pos];
        }

        // Now try digits 0..9
        let mut found = false;
        for i in 0..10 {
            if solved(f, i) + solved(s_part, i) == solved(ans_part, i) {
                println!("{}", i);
                found = true;
                break;
            }
        }
        if !found {
            println!("NA");
        }
    }
}
