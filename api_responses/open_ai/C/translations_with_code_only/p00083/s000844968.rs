// main.rs
// Translated from the provided C program.
// Notes on translation decisions preserving C semantics:
// - The C program declares `t` once (initialized to 0) outside the input loop and never resets it
//   at the top of each iteration. To preserve that behavior exactly, `t` is kept outside the loop
//   and not reinitialized per-record.
// - The C program declares `d` without an explicit initializer. Reading an uninitialized local in C
//   is undefined behavior; to avoid UB in Rust, `d` is initialized to 0. In typical inputs the C
//   code assigns `d` before it is read, so this preserves intended behavior while keeping Rust safe.
// - I/O behavior (whitespace, newlines, era labels, and the numeric output format) is preserved.
// - Arithmetic uses i32 to match typical C `int` behavior for these ranges.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse whitespace-separated tokens as i32.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    // Variables correspond to C's int a,b,c,d,t=0;
    let mut t: i32 = 0; // preserved across iterations (C initialized once outside loop)
    let mut d: i32 = 0; // initialized here to avoid undefined behavior on read

    let mut i = 0;
    while i + 2 < nums.len() {
        let mut a = nums[i];
        let b = nums[i + 1];
        let c = nums[i + 2];
        i += 3;

        // Note: `t` is not reset here to mimic the original C program
        if a <= 1868 && b <= 9 && c < 8 {
            print!("pre-meiji\n");
            d = 1;
            t = 1;
        } else if a <= 1912 {
            if a != 1912 {
                print!("meiji ");
                a = a - 1868 + 1;
                d = 0;
                t = 1;
            } else if a == 1912 {
                if b < 7 {
                    print!("meiji ");
                    a = a - 1868 + 1;
                    d = 0;
                    t = 1;
                } else if b >= 7 {
                    if c < 30 {
                        print!("meiji ");
                        a = a - 1868 + 1;
                        d = 0;
                        t = 1;
                    }
                }
            }
        }

        if a <= 1926 && t == 0 {
            if a != 1926 {
                print!("taisyo ");
                a = a - 1912 + 1;
                d = 0;
                t = 1;
            } else if a == 1926 {
                if b < 12 {
                    print!("taisyo ");
                    a = a - 1912 + 1;
                    d = 0;
                    t = 1;
                } else if b == 12 {
                    if c < 25 {
                        print!("taisyo ");
                        a = a - 1912 + 1;
                        d = 0;
                        t = 1;
                    }
                    if c >= 25 {
                        print!("showa ");
                        a = a - 1926 + 1;
                        d = 0;
                        t = 1;
                    }
                }
            }
        }

        if a <= 1989 && t == 0 {
            if a != 1989 {
                print!("showa ");
                a = a - 1926 + 1;
                d = 0;
                t = 1;
            } else if a == 1989 {
                if b == 1 {
                    if c < 8 {
                        print!("showa ");
                        a = a - 1926 + 1;
                        d = 0;
                        t = 1;
                    }
                }
            }
        }

        if a >= 1989 && t == 0 {
            if a != 1989 {
                print!("heisei ");
                a = a - 1989 + 1;
                d = 0;
            } else if a == 1989 {
                if b != 1 {
                    print!("heisei ");
                    a = a - 1989 + 1;
                    d = 0;
                } else if b == 1 {
                    if c >= 8 {
                        print!("heisei ");
                        a = a - 1989 + 1;
                        d = 0;
                    }
                }
            }
        }

        if d == 0 {
            println!("{} {} {}", a, b, c);
        }
        // Note: original C code does not reset t or d here; t persists, d remains as last set.
    }
}
