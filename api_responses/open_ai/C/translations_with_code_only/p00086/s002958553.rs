use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf in C reads from stdin progressively).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Tokenize by whitespace and parse integers.
    // We drop tokens that fail to parse; the C program assumes well-formed integer input.
    let tokens: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut pos: usize = 0;

    // Outer loop: repeat for each dataset until EOF (emulated by running out of tokens).
    'outer: loop {
        // If no more input left at the start of a dataset, behave like C: exit.
        if pos >= tokens.len() {
            break;
        }

        // graph indices 1..100 are used in the C code; index 0 is unused.
        // Initialize all to -1 as in the C code.
        let mut graph = vec![-1i32; 101];
        let mut odd_num: i32 = 0;

        // Inner loop: read pairs (from, to) until a "0 0" pair or EOF.
        'inner: loop {
            // Emulate scanf behavior for EOF: if we don't have another token, treat it as EOF
            // and exit the whole program (as the C code would do when scanf returns EOF).
            if pos >= tokens.len() {
                break 'outer;
            }
            let from = tokens[pos];
            pos += 1;

            if pos >= tokens.len() {
                // If there's no second number, treat as EOF and exit program.
                // (The original C program would have attempted to read and possibly lead to
                // undefined behavior if only one number remained; here we choose to exit.)
                break 'outer;
            }
            let to = tokens[pos];
            pos += 1;

            if from == 0 && to == 0 {
                // End of this dataset; evaluate graph.
                break 'inner;
            } else {
                // Update degrees. The C code uses graph[to] and graph[from] directly on a
                // fixed-size array of 101 ints. Accesses outside 1..100 would be undefined in C.
                // Here we guard to avoid UB in Rust; typical inputs are within 1..100.
                if (to as usize) >= 1 && (to as usize) <= 100 {
                    let idx = to as usize;
                    if graph[idx] == -1 {
                        graph[idx] = 1;
                    } else {
                        // Use wrapping_add to emulate typical two's-complement increment
                        // behavior rather than letting Rust panic on overflow. The original C
                        // signed overflow is undefined, but many platforms wrap; this mirrors that.
                        graph[idx] = graph[idx].wrapping_add(1);
                    }
                }
                if (from as usize) >= 1 && (from as usize) <= 100 {
                    let idx = from as usize;
                    if graph[idx] == -1 {
                        graph[idx] = 1;
                    } else {
                        graph[idx] = graph[idx].wrapping_add(1);
                    }
                }
            }
        }

        // Count odd-degree vertices. The C loop breaks at the first graph[i] == -1.
        for i in 1..=100 {
            if graph[i] == -1 {
                break;
            }
            if graph[i] % 2 != 0 {
                odd_num += 1;
            }
        }

        if odd_num == 0 || odd_num == 2 {
            // Preserve exact output lines from C.
            println!("OK");
        } else {
            println!("NG");
        }
    }
}
