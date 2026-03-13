use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse whitespace-separated tokens as integers,
    // similar to repeated scanf("%d", &n) calls.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    for token in input.split_whitespace() {
        // Parse each token as a 32-bit signed integer to match C's `int`.
        let n: i32 = match token.parse() {
            Ok(v) => v,
            // If a token isn't a valid integer we skip it. (Typical contest inputs are numeric.)
            Err(_) => continue,
        };

        // In the original C code, if n <= 0 the for-loop never terminates:
        // - n == 0: i stays 0 and the loop is infinite.
        // - n < 0: i decreases and the loop condition remains true, leading to an infinite loop.
        // To preserve that observable behavior (the program hangs and never prints for that input),
        // reproduce an infinite loop here.
        if n <= 0 {
            loop {}
        }

        // sum is a C 'long long int' -> 64-bit signed.
        let mut sum: i64 = 0;

        // Use i32 for `i` to match C's `int`. Use wrapping arithmetic to avoid Rust debug-panics
        // on overflow and to more closely emulate typical two's-complement behavior in C.
        let mut i: i32 = n;
        let limit: i32 = 600i32.wrapping_sub(n);

        // for (i = n; i <= 600 - n; i += n) { sum += i * i; }
        while i <= limit {
            let ii = i as i64;
            sum += ii * ii;
            i = i.wrapping_add(n);
        }

        // sum *= n;
        sum *= n as i64;

        // Print using the same formatting as printf("%lld\n", sum);
        println!("{}", sum);
    }
}
