use std::io::{self, Read};

fn main() {
    // Matches `int railroad[10] = {};` (zero-initialized)
    let mut railroad = [0i32; 10];
    let mut i: i32 = 0;

    // Read entire stdin, then parse ints like repeated `scanf("%d", &car)`
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for tok in input.split_whitespace() {
        // If a token isn't a valid int, C's scanf would stop; we stop as well.
        let car: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if car == 0 {
            // "leaving": i--; print railroad[i]
            i -= 1;
            // C would have undefined behavior if i is out of bounds; we assume valid input.
            let idx = i as usize;
            println!("{}", railroad[idx]);
        } else {
            // "arriving": railroad[i] = car; i++
            let idx = i as usize;
            railroad[idx] = car;
            i += 1;
        }
    }
}
