use std::io::{self, Read};

fn main() {
    // Read all whitespace-separated tokens like scanf("%s", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // f is a 5x5 int matrix, initialized to zero (matches C global int f[5][5];)
    let mut f = [[0i32; 5]; 5];

    // Build f from the 9 strings, mirroring the C loops and bit operations.
    // If a token is missing or too short, we substitute the byte '0' for safety.
    // The original C code would have undefined behavior in such cases; here we
    // choose a safe behavior that is compatible with typical valid inputs.
    for i in 0..9 {
        let s = tokens.get(i).unwrap_or(&"");
        let bytes = s.as_bytes();
        if i % 2 == 1 {
            // odd i: j = 0..4
            for j in 0..5 {
                let a = bytes.get(j).copied().unwrap_or(b'0').wrapping_sub(b'0') as i32;
                f[i / 2][j] |= a << 2;
                f[i / 2 + 1][j] |= a;
            }
        } else {
            // even i: j = 0..3
            for j in 0..4 {
                let a = bytes.get(j).copied().unwrap_or(b'0').wrapping_sub(b'0') as i32;
                f[i / 2][j] |= a << 1;
                f[i / 2][j + 1] |= a << 3;
            }
        }
    }

    // Reproduce the exact output behavior: print "R" then the sequence of moves, then newline.
    let mut out = String::new();
    out.push('R');

    // y, x, p as in C (use i32 to match signed arithmetic)
    let mut y: i32 = 0;
    let mut x: i32 = 1;
    let mut p: i32 = 1;

    // Helper bytes for "URDL"
    let letters = b"URDL";

    // do { ... } while (y!=0 || x!=0 || f[0][0]==6 && p==3);
    loop {
        // p = (p+2)%4;
        p = (p + 2) % 4;

        // while (!((f[y][x]>>(p=(p+1)%4))&1));
        loop {
            p = (p + 1) % 4;
            // index into f: convert to usize. This mirrors the C indexing; if y/x go out of range,
            // behavior is undefined in C. Here it will likely panic on invalid indices.
            let fyx = f[y as usize][x as usize];
            if ((fyx >> p) & 1) != 0 {
                break;
            }
        }

        // putchar("URDL"[p]);
        out.push(letters[p as usize] as char);

        // x += p ? 2-p : 0;
        if p != 0 {
            x = x + (2 - p);
        }
        // y += p==3 ? 0 : p-1;
        if p == 3 {
            // add 0
        } else {
            y = y + (p - 1);
        }

        // loop condition: while(y!=0||x!=0||f[0][0]==6&&p==3);
        if y == 0 && x == 0 && !(f[0][0] == 6 && p == 3) {
            break;
        }
    }

    out.push('\n');
    print!("{}", out);
}
