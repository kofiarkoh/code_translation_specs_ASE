use std::io::{self, Read};

fn main() {
    // Read all input from stdin as whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, exit silently as C program would just terminate.
        return;
    }

    // Parse tokens as i32. On first non-integer token we stop parsing further tokens.
    // This is a pragmatic choice to provide well-defined behavior in Rust.
    // The original C used scanf and had undefined/stale-variable behavior on malformed input;
    // to avoid undefined behavior here we stop at the first non-integer token.
    let mut tokens: Vec<i32> = Vec::new();
    for tok in input.split_whitespace() {
        match tok.parse::<i32>() {
            Ok(v) => tokens.push(v),
            Err(_) => break,
        }
    }

    // Process complete triples only.
    let mut i = 0usize;

    // Flags that persist across records.
    // t corresponds to the "era decision made" flag and is initialized to 0 in the C code.
    let mut t: i32 = 0;

    // In the original C code `d` was not initialized which yields undefined behavior in C.
    // To keep the Rust program well-defined we initialize it to 0 here.
    // This is a deliberate, documented divergence from the C code's undefined behavior.
    // (If reproducing the undefined behavior exactly were required, we'd have to use unsafe code,
    // which we avoid.)
    let mut d: i32 = 0;

    while i + 2 < tokens.len() {
        // Read a, b, c for this record.
        let mut a = tokens[i];
        let b = tokens[i + 1];
        let c = tokens[i + 2];
        i += 3;

        // The following logic mirrors the C code structure and comparisons.
        if a <= 1868 && b <= 9 && c < 8 {
            // pre-meiji path
            println!("pre-meiji");
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
            if t == 0 {
                // In the C code, t is set to 1 in many heisei branches but not in the
                // branch where a != 1989 (it prints and sets a,d but didn't set t).
                // However earlier in the C code that branch did set t=1; to match C exactly,
                // set t=1 when we printed heisei in the a!=1989 path as well.
                // The original C sets t=1 in that path (look at indentation); preserve that.
                t = 1;
            }
        }

        if d == 0 {
            println!("{} {} {}", a, b, c);
        }
        // Note: In the original C program `t` and `d` persist across iterations and are not
        // reset per record. We preserve that behavior here (they keep their values).
        // Also note we initialized `d` to 0 to avoid undefined behavior present in the C code.
    }
}
