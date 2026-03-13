use std::io::{self, Read};

fn main() {
    // Corresponds to: int countArray[101]; memset(..., 0);
    // Use i32 for counts to mirror C 'int'. Use wrapping_add to avoid
    // Rust debug panics on overflow and to approximate typical two's-complement wrap behavior.
    let mut counts = [0i32; 101];

    // Read all stdin (scanf("%d", ...) in the C version reads ints until EOF).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Tokenize by whitespace and parse integers.
    // Note: The original C program uses scanf("%d", &number) which on non-integer input
    // would leave the token unread and (in the original loop) cause an infinite loop.
    // Here we choose to skip tokens that fail to parse to avoid locking; this matches
    // the typical intended use (well-formed integer input).
    for tok in input.split_whitespace() {
        if let Ok(number) = tok.parse::<i32>() {
            // The C code indexes countArray[number] without any bounds check.
            // That is undefined behavior if number is outside 0..=100. To remain safe
            // in Rust we only count numbers in the range 0..=100 (typical for this task).
            if (0..=100).contains(&number) {
                let idx = number as usize;
                counts[idx] = counts[idx].wrapping_add(1);
            } else {
                // If input contains out-of-range values, the original C program's behavior
                // would be undefined. We silently ignore them here.
            }
        } else {
            // Skip non-integer tokens (see note above).
        }
    }

    // Find mx_number.
    // Corresponds to:
    // int mx_number = 0;
    // for(number = 100; number > 0; --number)
    //   if(countArray[number] > countArray[mx_number])
    //     mx_number = number;
    let mut mx_number: usize = 0;
    for number in (1..=100).rev() {
        if counts[number] > counts[mx_number] {
            mx_number = number;
        }
    }

    // Print numbers from 1..=mx_number that tie for the maximum frequency,
    // each followed by a newline (printf("%d\n", number);).
    for number in 1..=mx_number {
        if counts[number] == counts[mx_number] {
            println!("{}", number);
        }
    }
}
