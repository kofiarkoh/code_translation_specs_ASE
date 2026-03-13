use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf in the C original reads whitespace-separated ints)
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, just exit (matches program doing nothing further)
        return;
    }

    // Split by whitespace into tokens
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Process tokens in pairs (x, h). This mirrors scanf("%d%d", &x, &h) behavior
    let mut i = 0;
    while i + 1 < tokens.len() {
        // Try to parse as 32-bit signed ints to match typical C `int`
        let x = match tokens[i].parse::<i32>() {
            Ok(v) => v,
            Err(_) => break, // If parsing fails, stop further processing (similar to scanf failing)
        };
        let h = match tokens[i + 1].parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // If x + h == 0 then loop should terminate without printing (same as the C comma/while behavior)
        if x.wrapping_add(h) == 0 {
            break;
        }

        // Compute in f64 like the C expression: sqrt(x*x + 4*h*h)*x + x*x
        // Cast to f64 first to avoid integer overflow differences and to match the double math in C.
        let xf = x as f64;
        let hf = h as f64;
        let val = (xf * xf + 4.0 * hf * hf).sqrt() * xf + xf * xf;

        // printf("%lf\n", ...) prints a double with 6 digits after the decimal by default.
        // Use "{:.6}" to produce the same formatting.
        println!("{:.6}", val);

        i += 2;
    }
}
