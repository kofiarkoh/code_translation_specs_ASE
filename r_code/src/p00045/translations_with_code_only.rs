use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf in the C program reads from stdin until EOF).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let n = bytes.len();
    let mut i = 0;

    // Corresponding C types:
    // long long int sum = 0;
    // int num_sum = 0;
    // int times = 0;
    //
    // We use wrapping arithmetic when updating these to emulate typical
    // two's-complement wraparound behavior that often occurs in practice
    // when C signed integers overflow. (C signed overflow is technically
    // undefined, but many programs rely on wrapping; to be conservative we
    // emulate wrap behavior.)
    let mut sum: i64 = 0;
    let mut num_sum: i32 = 0;
    let mut times: i32 = 0;

    // We attempt to emulate scanf("%d,%d", &prise, &num) behaviour in a
    // straightforward, streaming way:
    //
    // - For the first %d scanf skips leading whitespace, then parses an
    //   optional '+'/'-' and digits.
    // - Then scanf expects a literal comma (no skipping of whitespace before it).
    // - Then for the second %d scanf skips leading whitespace, then parses
    //   optional sign and digits.
    //
    // If parsing fails at any point we advance a little to avoid an infinite
    // loop on malformed input. This is a pragmatic choice to keep behavior
    // robust while preserving expected behavior on well-formed inputs.
    while i < n {
        // Skip leading whitespace before first %d
        while i < n && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= n {
            break;
        }

        let start = i;

        // Parse optional sign for first integer
        let mut sign = 1i64;
        if bytes[i] == b'+' {
            i += 1;
        } else if bytes[i] == b'-' {
            sign = -1;
            i += 1;
        }

        // Need at least one digit
        if i >= n || !bytes[i].is_ascii_digit() {
            // Can't parse first integer here; advance from start to avoid infinite loop
            i = start + 1;
            continue;
        }

        // Parse digits for first integer (produce i64 accumulator)
        let mut val1: i64 = 0;
        while i < n && bytes[i].is_ascii_digit() {
            // safe to do in i64; large numbers will be truncated when cast to i32 below
            val1 = val1 * 10 + (bytes[i] - b'0') as i64;
            i += 1;
        }
        val1 *= sign;

        // Expect a literal comma immediately (no skipping of whitespace here).
        if i >= n {
            // Reached EOF before comma; cannot form a pair -> stop (scanf would return 1 != EOF,
            // but there is no second value; pragmatically break).
            break;
        }
        if bytes[i] != b',' {
            // Mismatch; advance from start to avoid infinite loop and try again
            i = start + 1;
            continue;
        }
        // consume comma
        i += 1;

        // For the second %d scanf skips leading whitespace
        while i < n && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= n {
            // EOF after comma -> cannot form pair
            break;
        }

        let start2 = i;
        // Parse optional sign for second integer
        let mut sign2 = 1i64;
        if bytes[i] == b'+' {
            i += 1;
        } else if bytes[i] == b'-' {
            sign2 = -1;
            i += 1;
        }

        // Need at least one digit for second integer
        if i >= n || !bytes[i].is_ascii_digit() {
            // Fail to parse second integer; advance from first start to avoid infinite loop
            i = start + 1;
            continue;
        }

        // Parse digits for second integer
        let mut val2: i64 = 0;
        while i < n && bytes[i].is_ascii_digit() {
            val2 = val2 * 10 + (bytes[i] - b'0') as i64;
            i += 1;
        }
        val2 *= sign2;

        // Now we have both parsed integers; in C they are stored into 'int' variables.
        // Emulate that by casting to i32 (this will wrap/truncate like C in typical two's-complement).
        let prise = val1 as i32;
        let num = val2 as i32;

        // Update sum: sum += (long long)prise * (long long)num;
        // Use wrapping operations to emulate wraparound semantics.
        sum = sum.wrapping_add((prise as i64).wrapping_mul(num as i64));
        num_sum = num_sum.wrapping_add(num);
        times = times.wrapping_add(1);
    }

    // Compute average as double: (double)(num_sum)/times
    // Convert i32 to f64; if times == 0 (or wrapped to 0) this yields inf/nan consistent with IEEE double.
    let avg = (num_sum as f64) / (times as f64);

    // Print exactly like printf("%lld\n%.0lf\n", sum, avg);
    // First line: sum as signed 64-bit decimal, newline.
    // Second line: avg formatted with zero digits after decimal, newline.
    // Using "{:.0}" for the floating formatting (rounding as Rust implements).
    print!("{}\n{:.0}\n", sum, avg);
}
