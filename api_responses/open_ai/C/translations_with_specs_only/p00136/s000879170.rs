use std::io::{self, Read};

fn main() {
    // Read all stdin and split by whitespace into tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On I/O error, behave conservatively: print six empty buckets.
        print_buckets([0usize; 6]);
        return;
    }

    let mut tokens = input.split_whitespace();

    // Parse the first token as the integer n.
    // The Specification treats an unparsable initial integer as "undefined".
    // To remain well-defined in Rust while staying narrow and conservative, treat
    // a missing or invalid first token as n = 0 (process zero values).
    let n: i64 = match tokens.next() {
        Some(tok) => tok.parse().unwrap_or(0),
        None => 0,
    };

    // If n is <= 0, we process zero values (matching typical C behavior where a negative loop count results in no iterations).
    let mut to_process: usize = if n > 0 { n as usize } else { 0 };

    // Counters for the six buckets: indices 0..5 correspond to buckets 1..6.
    let mut cnt = [0usize; 6];

    // Process tokens until we've successfully parsed `to_process` doubles or run out of tokens.
    // For tokens that do not parse as f64, skip them; only successfully parsed doubles affect counters.
    // This is a narrow, reasonable interpretation consistent with "only successfully read doubles affect counters".
    while to_process > 0 {
        let tok = match tokens.next() {
            Some(t) => t,
            None => break, // no more input tokens
        };

        let v: f64 = match tok.parse() {
            Ok(x) => x,
            Err(_) => continue, // skip unparsable token
        };

        // NaN must not match any bucket per Specification.
        if v.is_nan() {
            // Do not decrement to_process here: to_process counts how many successful reads we still want.
            // However, the spec indicates only successfully read doubles affect counters; since parsing succeeded,
            // it is considered a successfully read double in this implementation, but NaN should not increment any bucket.
            // We still count it as one of the processed values (it was a successfully parsed double).
            to_process -= 1;
            continue;
        }

        // Classify according to the precise intervals.
        // Bucket 1: v < 165.0
        // Bucket 2: 165.0 <= v < 170.0
        // Bucket 3: 170.0 <= v < 175.0
        // Bucket 4: 175.0 <= v < 180.0
        // Bucket 5: 180.0 <= v < 185.0
        // Bucket 6: v >= 185.0
        if v < 165.0 {
            cnt[0] += 1;
        } else if v < 170.0 {
            cnt[1] += 1;
        } else if v < 175.0 {
            cnt[2] += 1;
        } else if v < 180.0 {
            cnt[3] += 1;
        } else if v < 185.0 {
            cnt[4] += 1;
        } else {
            // v >= 185.0 covers +inf as well
            cnt[5] += 1;
        }

        to_process -= 1;
    }

    print_buckets(cnt);
}

// Print exactly six lines "i:***\n" where the number of '*' equals count for bucket i.
fn print_buckets(cnt: [usize; 6]) {
    for (i, &c) in cnt.iter().enumerate() {
        // bucket indices are 1-based in output
        print!("{}:", i + 1);
        if c > 0 {
            // Efficiently create a string of c asterisks
            let s: String = std::iter::repeat('*').take(c).collect();
            print!("{}", s);
        }
        println!();
    }
}
