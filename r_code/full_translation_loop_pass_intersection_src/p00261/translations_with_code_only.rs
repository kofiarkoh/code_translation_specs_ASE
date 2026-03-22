use std::io::{self, Read};

const A: usize = 0;
const X: usize = 1;
const Y: usize = 2;
const Z: usize = 3;
const W: usize = 4;
const B: usize = 5;
const NA: usize = 6;

fn main() {
    // Equivalent to the C global `map`.
    let map: [[usize; 2]; 6] = [
        [X, Y],
        [NA, Z],
        [X, NA],
        [W, B],
        [B, Y],
        [Y, X],
    ];

    // C uses `scanf("%s", path)` repeatedly: token-based, whitespace-delimited.
    // We'll read all stdin and split by whitespace to emulate that behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        // In C: if (path[0] == '#') break;
        if token.as_bytes().first() == Some(&b'#') {
            break;
        }

        let bytes = token.as_bytes();
        let mut city: usize = A;
        let mut i: usize = 0;

        // In C: while (path[i] != '\0') { ... }
        while i < bytes.len() {
            // C does: path[i] - '0' and uses it as index 0/1.
            // If the character isn't '0' or '1', C would index out of bounds (UB).
            // We emulate the "intended" behavior by stopping the walk on invalid input.
            let idx = match bytes[i] {
                b'0' => 0usize,
                b'1' => 1usize,
                _ => break,
            };

            let next = map[city][idx];
            if next == NA {
                break;
            }
            city = next;
            i += 1;
        }

        if city == B {
            out.push_str("Yes\n");
        } else {
            out.push_str("No\n");
        }
    }

    print!("{out}");
}
