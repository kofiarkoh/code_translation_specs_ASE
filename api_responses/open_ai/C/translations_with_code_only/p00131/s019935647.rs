// Translated from the provided C program.
// Notes on translation choices:
// - The C program used fixed-size arrays s[12][12] and a[12][12]. We replicate that and
//   initialize them to zero. The original C code left some entries possibly
//   uninitialized and performed some accesses that in C could be out-of-bounds;
//   initializing to zero is the safer, likely intended behavior for edge cells.
// - Integer updates use wrapping_add to avoid panics on overflow and to more closely
//   emulate low-level wraparound behaviour that C implementations might exhibit.
// - Boundary accesses (j-1, j+1, i+2) are guarded to stay within the 12x12 arrays.
//   The original C code depended on the 12x12 padding to avoid going past the array
//   bounds when using indices 0..9 for the core 10x10 grid; we keep that layout
//   and ensure we do not index out of bounds in Rust.
// - Input/output formatting is preserved exactly: each printed integer is followed by
//   a space, and each row ends with a newline. No extra blank lines are printed.

use std::io::{self, Read};

fn main() {
    // Read all integers from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().expect("parse int"));

    // Read number of test cases (like scanf("%d",&n); while(n--){...})
    let n = match iter.next() {
        Some(v) => v,
        None => return,
    };
    // For safety, if n is negative we treat as 0 (the C code's while(n--) with negative n
    // would have implementation-defined/UB behavior). This matches typical intended usage.
    let cases = if n >= 0 { n as usize } else { 0usize };

    for _case in 0..cases {
        // s and a are 12x12 as in C. Initialize to zero.
        let mut s = [[0i32; 12]; 12];
        let mut a = [[0i32; 12]; 12];

        // Read 10x10 integers into s[0..9][0..9].
        for i in 0..10 {
            for j in 0..10 {
                if let Some(val) = iter.next() {
                    s[i][j] = val;
                } else {
                    // If input missing, treat remaining as zero (robustness).
                    s[i][j] = 0;
                }
            }
        }

        // Main logic: for(i=0;i<9;i++) for(j=0;j<10;j++) ...
        for i in 0..9 {
            for j in 0..10 {
                if s[i][j] % 2 == 1 {
                    // a[i+1][j] = 1;
                    a[i + 1][j] = 1;
                    // Increment neighbors, using wrapping_add to mimic low-level wraparound.
                    s[i][j] = s[i][j].wrapping_add(1);
                    s[i + 1][j] = s[i + 1][j].wrapping_add(1);
                    if j >= 1 {
                        s[i + 1][j - 1] = s[i + 1][j - 1].wrapping_add(1);
                    }
                    if j + 1 < 12 {
                        s[i + 1][j + 1] = s[i + 1][j + 1].wrapping_add(1);
                    }
                    if i + 2 < 12 {
                        s[i + 2][j] = s[i + 2][j].wrapping_add(1);
                    }
                }
            }
        }

        // Print a[0..9][0..9] with the same formatting as the C code:
        // for(i=0;i<10;i++){for(j=0;j<10;j++)printf("%d ",a[i][j]);printf("\n");}
        for i in 0..10 {
            for j in 0..10 {
                print!("{} ", a[i][j]);
            }
            println!();
        }
    }
}
