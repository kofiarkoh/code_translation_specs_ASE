// magic.rs
//
// Translated from the provided C program according to the SPEC.
// Notes:
// - The SPEC requires that for each non-zero n (1..=16) the program
//   fills an n×n logical matrix (backed by a fixed 16×16 array) with
//   the integers 1..n^2 according to the placement algorithm and prints
//   the matrix with each entry in a 4-character-wide field.
// - The original C code did `y = x = hn; y++;` (i.e. a plain increment
//   of y) before entering the placement loop. That causes incorrect
//   behavior for some inputs (notably n == 1) because the increment was
//   not reduced modulo n. The SPEC describes the intended behavior
//   (produce the numbers 1..n^2 in the n×n logical matrix). To satisfy
//   the SPEC we perform the initial y increment modulo n:
//       y = (hn + 1) % n
//   This deviates from the literal C code in that one plain increment
//   was changed to a modulo increment; the change is necessary to
//   preserve the SPEC guarantee that the printed matrix contains the
//   numbers 1..n^2 for valid n.
// - We assume inputs meet the SPEC precondition (1 <= n <= 16 for each
//   non-zero processed n). We assert this to avoid undefined behavior
//   analogous to out-of-bounds writes in the C program.
//
// Behavior:
// - Read whitespace-separated integers from stdin.
// - For each integer n:
//     - if n == 0 -> terminate normally.
//     - assert 1 <= n <= 16 (per SPEC precondition for safe execution).
//     - construct/fill the logical n×n portion of a fixed 16×16 array
//       using the placement algorithm, then print n lines with n
//       integers each, each printed in a width-4 field, rows separated
//       by newlines.

use std::io::{self, Read};

fn main() {
    // Read all input tokens (whitespace separated) like scanf("%d", ...)
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit; this mirrors that the original C
        // program expected valid reads and had undefined behavior otherwise.
        return;
    }

    let mut tokens = input.split_whitespace();

    // Fixed backing storage 16x16, initialized to 0.
    // We'll reuse this buffer for each n, but reinitialize the logical n×n region.
    // Use i32 to match C int semantics for the values printed.
    let mut magic = [[0i32; 16]; 16];

    while let Some(tok) = tokens.next() {
        // Parse next integer token. If parse fails, stop processing (similar to
        // the unspecified behavior of the original C program on malformed input).
        let n_val: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if n_val == 0 {
            break;
        }

        // Per SPEC, safe execution requires 1 <= n <= 16. Enforce this to avoid
        // out-of-bounds accesses. If violated, we panic which is preferable to
        // performing unsafe out-of-bounds writes.
        assert!(
            (1..=16).contains(&n_val),
            "n out of supported range (1..=16) as required by SPEC: {}",
            n_val
        );

        let n = n_val as usize;

        // Clear the logical n×n portion of the buffer.
        for i in 0..n {
            for j in 0..n {
                magic[i][j] = 0;
            }
        }

        // Placement algorithm (Siamese-like). Matches SPEC intention:
        // fill numbers 1..n^2 into the logical n×n.
        let hn = n / 2;

        // Coordinates: x, y
        let mut x = hn;
        // NOTE: The original C code did `y = x = hn; y++;` without applying
        // modulo. That produces incorrect result for n==1 (and is generally
        // fragile). To satisfy the SPEC guarantee that the printed matrix
        // contains 1..n^2 we apply the increment modulo n here.
        let mut y = (hn + 1) % n;

        let mut number: i32 = 1;
        let max_number = (n * n) as i32;

        while number <= max_number {
            if magic[y][x] != 0 {
                // move down one and left one (with wrapping) if current cell occupied
                y = (y + 1 + n) % n;
                x = (x + n - 1) % n;
                continue;
            }
            magic[y][x] = number;
            y = (y + 1) % n;
            x = (x + 1) % n;
            number += 1;
        }

        // Print the logical n×n, each entry in a 4-character field, rows on separate lines.
        for i in 0..n {
            for j in 0..n {
                // Right-aligned in width 4, no additional separators.
                print!("{:4}", magic[i][j]);
            }
            println!();
        }
    }
}
