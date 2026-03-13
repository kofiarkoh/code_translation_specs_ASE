use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit normally with code 0 as per spec.
        return;
    }

    // The source C program expected numbers separated by commas.
    // We split on commas and trim whitespace around tokens. This is a
    // narrow and reasonable interpretation of the scanf("%d,%d,...") style.
    let tokens: Vec<&str> = input.split(',').map(str::trim).collect();

    let mut idx: usize = 0;
    let n = tokens.len();

    // Helper to parse next token as i32. On parse failure, we treat it as malformed
    // input and stop processing further records (a conservative choice consistent with
    // the specification's note about undefined behavior for malformed input).
    let mut parse_i32 = |i: usize| -> Option<i32> {
        if i >= n {
            return None;
        }
        // Empty token is a parse error.
        let s = tokens[i];
        if s.is_empty() {
            return None;
        }
        match s.parse::<i32>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    };

    // Process records: each record is 10 ints (r[0..9]) followed by v1 and v2.
    loop {
        // If fewer than 10 tokens remain when starting a new record, treat as EOF
        // while attempting to read the ten-bucket sequence and exit normally.
        if idx + 10 > n {
            break;
        }

        // Parse 10 bucket integers.
        let mut r: [i32; 10] = [0; 10];
        let mut ok = true;
        for i in 0..10 {
            match parse_i32(idx + i) {
                Some(v) => r[i] = v,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            break;
        }
        idx += 10;

        // Now attempt to parse v1 and v2. The original C program did not
        // robustly check this read; behavior is undefined in that case.
        // Here we conservatively stop processing if v1 or v2 are missing/malformed.
        if idx + 2 > n {
            break;
        }
        let v1 = match parse_i32(idx) {
            Some(v) => v,
            None => break,
        };
        let v2 = match parse_i32(idx + 1) {
            Some(v) => v,
            None => break,
        };
        idx += 2;

        // Compute S = sum(r[0..9]) as f64 (sum in i32 then cast).
        let s_i32: i32 = r.iter().copied().sum();
        let s = s_i32 as f64;

        // Compute x = (double)v1 / (double)(v1 + v2) * S
        // Cast to f64 to emulate C double behavior (IEEE handling of zero denom).
        let denom = (v1 as i32).wrapping_add(v2 as i32) as f64; // wrapping_add matches narrow interpretation if overflow occurs; but wrapping is only for i32 add.
        // Use f64 division (may produce inf/NaN if denom == 0).
        let x = (v1 as f64) / denom * s;

        // Scan buckets accumulating running sum and find smallest 1-based index k
        // such that x <= running_sum. If none found, output 0.
        let mut running_sum = 0f64;
        let mut out_index = 0i32;
        for (i, &val) in r.iter().enumerate() {
            running_sum += val as f64;
            if x <= running_sum {
                out_index = (i + 1) as i32; // 1-based index
                break;
            }
        }

        // Print result as a single integer followed by a newline.
        println!("{}", out_index);
    }

    // Exit normally with code 0.
}
