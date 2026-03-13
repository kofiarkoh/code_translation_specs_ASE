// translated from the provided C program
// Preserves the C program's behavior (reading 12-token grids, converting '0'/'1' chars,
// counting connected components via 4-neighbor flood-fill, printing count per grid).
// Notes:
// - The original C used a global `char map[16][16]` with zero padding and recursive
//   deletion. To avoid undefined behavior from out-of-bounds accesses in C, this Rust
//   translation enforces bounds checks that emulate the intended sentinel-zero padding.
// - We emulate the C's byte-level subtraction `map[i][j+1] -= '0'` using wrapping_sub on u8,
//   so that non-digit zero bytes behave similarly (they become 0.wrapping_sub(48) -> 208).
use std::io::{self, Read};

fn delete_island(map: &mut [[u8; 16]; 16], x: isize, y: isize) {
    // Bounds-check to emulate sentinel-zero padding and avoid UB.
    if x < 0 || x >= 16 || y < 0 || y >= 16 {
        return;
    }
    let (xi, yi) = (x as usize, y as usize);
    if map[xi][yi] == 0 {
        return;
    }
    map[xi][yi] = 0;
    delete_island(map, x - 1, y);
    delete_island(map, x + 1, y);
    delete_island(map, x, y - 1);
    delete_island(map, x, y + 1);
}

fn main() {
    // Read all stdin into a buffer and manage our own cursor to emulate scanf behavior.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut pos: usize = 0;
    let len = input.len();

    // Helper to check ASCII whitespace
    let is_ws = |b: u8| b.is_ascii_whitespace();

    // next_token() behaves like scanf("%s", ...): it skips leading whitespace,
    // then returns the bytes up to the next whitespace (not including it).
    let mut next_token = || -> Option<&[u8]> {
        // skip leading whitespace
        while pos < len && is_ws(input[pos]) {
            pos += 1;
        }
        if pos >= len {
            return None;
        }
        let start = pos;
        while pos < len && !is_ws(input[pos]) {
            pos += 1;
        }
        Some(&input[start..pos])
    };

    // next_char_no_skip() behaves like scanf("%*c"): it consumes the very next byte
    // (even if it's whitespace). Returns true if a byte was consumed.
    let mut consume_one_char = || -> bool {
        if pos < len {
            pos += 1;
            true
        } else {
            false
        }
    };

    // The map is a 16x16 byte grid, zero-initialized to emulate `char map[16][16] = {0};`
    let mut map = [[0u8; 16]; 16];

    // Outer loop: while (scanf("%s", &map[0][1]) != EOF)
    loop {
        // Read the first token for the first row
        let first = match next_token() {
            Some(t) => t,
            None => break, // EOF
        };

        // Clear map for a new case (the C code reuses the same global array but overwrites relevant cells;
        // zeroing here mirrors the initial state and the C initializer).
        for r in 0..16 {
            for c in 0..16 {
                map[r][c] = 0;
            }
        }

        // Copy first token into map[0][1..], as bytes (like scanf into &map[0][1]).
        // We copy up to available space; the C code would write a trailing '\0' but
        // the subsequent conversion loop only subtracts '0' from the first 12 columns.
        for (i, &b) in first.iter().enumerate().take(15) {
            map[0][1 + i] = b;
        }

        // Read the remaining 11 rows: for (i = 1; i < 12; i++) scanf("%s", &map[i][1]);
        let mut fail_mid = false;
        for row in 1..12 {
            match next_token() {
                Some(tkn) => {
                    for (i, &b) in tkn.iter().enumerate().take(15) {
                        map[row][1 + i] = b;
                    }
                }
                None => {
                    // EOF in the middle of a grid: mimic C which would also encounter EOF.
                    fail_mid = true;
                    break;
                }
            }
        }
        if fail_mid {
            break;
        }

        // Convert characters to numeric values: for (i = 0; i < 12; i++) for (j = 0; j < 12; j++) map[i][j+1] -= '0';
        for i in 0..12 {
            for j in 0..12 {
                // use wrapping_sub to emulate byte-underflow behavior for non-digit zeros
                map[i][j + 1] = map[i][j + 1].wrapping_sub(b'0');
            }
        }

        // Count islands: for each cell in rows 0..11, cols 1..12
        let mut count = 0;
        for i in 0..12 {
            for j in 0..12 {
                if map[i][j + 1] == 1 {
                    count += 1;
                    delete_island(&mut map, i as isize, (j + 1) as isize);
                }
            }
        }

        // Print result like printf("%d\n", count);
        println!("{}", count);

        // scanf("%*c") : consume one character (even if whitespace). Ignore if EOF.
        let _ = consume_one_char();
    }
}
