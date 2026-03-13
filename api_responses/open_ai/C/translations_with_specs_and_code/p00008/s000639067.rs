use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Read entire stdin as whitespace-separated tokens.
    // Spec guarantees tokens are valid decimal integers; if a token fails to parse,
    // behavior is unspecified in the original C program. Here we stop processing
    // further tokens on the first malformed token (avoiding panic).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut output = String::new();

    for token in input.split_whitespace() {
        // Parse each token as a signed 32-bit integer (matches C 'int' assumption).
        let n: i32 = match token.parse() {
            Ok(v) => v,
            // On parse failure we stop processing further tokens.
            // The original C program using scanf("%d", &n) does not handle
            // non-integer tokens; this translation follows the spec's assumption
            // that tokens are valid integers and avoids panicking on malformed input.
            Err(_) => break,
        };

        // Count ordered 4-tuples (a,b,c,d) with each in 0..=9 and a+b+c+d == n.
        let mut count: i32 = 0;
        for a in 0..=9 {
            for b in 0..=9 {
                for c in 0..=9 {
                    for d in 0..=9 {
                        if a + b + c + d == n {
                            count += 1;
                        }
                    }
                }
            }
        }

        // Write "<count>\n" for each input integer.
        output.push_str(&format!("{}\n", count));
    }

    // Flush the accumulated output to stdout.
    print!("{}", output);

    Ok(())
}
