// translated from the given C program according to the provided SPEC.
// Preserves semantics: reads 9 whitespace-separated tokens from stdin,
// builds a 5x5 adjacency bitmask (bits: 0=U,1=R,2=D,3=L), performs the
// deterministic clockwise-scanning walk starting at (0,1) facing R,
// prints "R" followed by the sequence of steps and a trailing newline.

use std::io::{self, Read};

fn main() {
    // Read all stdin and split on whitespace to get tokens as C's scanf("%s") would.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, behave conservatively and exit (SPEC says behavior undefined for malformed input).
        return;
    }
    let toks: Vec<&str> = input.split_whitespace().collect();
    if toks.len() < 9 {
        // SPEC assumes 9 tokens; for fewer tokens behavior is unspecified. Exit early.
        return;
    }

    // f[y][x] bitmask: bit 0 = U, 1 = R, 2 = D, 3 = L
    let mut f = [[0u8; 5]; 5];

    for i in 0..9 {
        let s = toks[i].as_bytes();
        if i % 2 == 1 {
            // vertical edges: token has at least 5 chars, indexed j=0..4
            let row = i / 2;
            for j in 0..5 {
                // per SPEC/C code, we use only the first characters; assume they are '0' or '1'
                let a = s[j].wrapping_sub(b'0') & 1;
                // top cell gets D (bit 2), bottom cell gets U (bit 0)
                f[row][j] |= a << 2;
                f[row + 1][j] |= a << 0;
            }
        } else {
            // horizontal edges: token has at least 4 chars, indexed j=0..3
            let row = i / 2;
            for j in 0..4 {
                let a = s[j].wrapping_sub(b'0') & 1;
                // left cell gets R (bit 1), right cell gets L (bit 3)
                f[row][j] |= a << 1;
                f[row][j + 1] |= a << 3;
            }
        }
    }

    // Build output string starting with initial orientation 'R'
    let mut out = String::new();
    out.push('R');

    // walker state
    let mut y: i32 = 0;
    let mut x: i32 = 1;
    let mut p: i32 = 1; // 0=U,1=R,2=D,3=L

    // direction characters for indexing by p
    let dirs = ['U', 'R', 'D', 'L'];

    // do-while loop: execute body at least once
    loop {
        // p = (p + 2) % 4;
        p = (p + 2) % 4;
        // scan clockwise by repeatedly incrementing p until an available edge is found
        loop {
            p = (p + 1) % 4;
            let mask = f[y as usize][x as usize];
            if ((mask >> (p as u8)) & 1) != 0 {
                break;
            }
            // else keep scanning
        }

        // append the direction letter
        out.push(dirs[p as usize]);

        // update coordinates consistent with the original arithmetic:
        // x += p ? 2-p : 0;
        // y += p==3 ? 0 : p-1;
        if p != 0 {
            x += 2 - p;
        }
        if p == 3 {
            // y += 0;
        } else {
            y += p - 1;
        }

        // termination: continue while (y!=0 || x!=0 || (f[0][0]==6 && p==3));
        // so break when y==0 && x==0 && !(f[0][0]==6 && p==3)
        let at_origin = (y == 0) && (x == 0);
        let special = (f[0][0] == 6) && (p == 3);
        if at_origin && !special {
            break;
        }
        // else continue loop
    }

    // print with trailing newline as in the original program
    println!("{}", out);
}
