use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, behave like EOF / terminate.
        return;
    }

    let mut tokens = input.split_whitespace();
    // Outer loop: process datasets until EOF or incomplete dataset encountered.
    loop {
        // graph[0] unused; indices 1..=100 used. Initialized to -1 meaning "not seen".
        let mut graph = [-1i32; 101];
        // Track the last successfully read pair (mirrors C code's behavior).
        let mut last_from = -1i32;
        let mut last_to = -1i32;

        // Inner loop: read pairs for the current dataset.
        loop {
            // Attempt to get next two tokens (one pair).
            let a_tok = tokens.next();
            let b_tok = tokens.next();

            // If we can't get a full pair, treat as EOF / read failure.
            if a_tok.is_none() || b_tok.is_none() {
                // According to the specification, if EOF occurs before a terminating "0 0"
                // for the current dataset, the program terminates without producing a result
                // for that incomplete dataset.
                return;
            }

            // Try parsing tokens as integers. On parse failure, treat as read failure / EOF.
            let from = match a_tok.unwrap().parse::<i32>() {
                Ok(v) => v,
                Err(_) => return,
            };
            let to = match b_tok.unwrap().parse::<i32>() {
                Ok(v) => v,
                Err(_) => return,
            };

            last_from = from;
            last_to = to;

            // Sentinel pair terminates current dataset.
            if from == 0 && to == 0 {
                break;
            } else {
                // Update degrees. This mirrors the C code semantics:
                // - graph[i] == -1 means not-seen; first occurrence sets to 1.
                // - subsequent occurrences increment.
                // - self-loops (from == to) will be processed twice, resulting in +2 degree.
                // Note: following the specification, vertex ids are expected in 1..=100.
                // If an id is outside that range, indexing will panic (this mirrors the C
                // program's undefined behavior for out-of-range ids).
                let fi = from as usize;
                let ti = to as usize;

                graph[ti] = if graph[ti] == -1 { 1 } else { graph[ti] + 1 };
                graph[fi] = if graph[fi] == -1 { 1 } else { graph[fi] + 1 };
            }
        } // end inner loop: dataset terminated by 0 0

        // Evaluate the dataset:
        // The original C code inspects graph[1..=100] but breaks on the first index
        // whose degree is -1. We preserve that behavior: only a contiguous prefix
        // of vertex ids starting at 1 is considered.
        let mut odd_num = 0i32;
        for i in 1..=100 {
            if graph[i] == -1 {
                break;
            }
            if graph[i] % 2 != 0 {
                odd_num += 1;
            }
        }

        if odd_num == 0 || odd_num == 2 {
            // Exactly match output formatting: "OK\n"
            println!("OK");
        } else {
            println!("NG");
        }

        // Continue to next dataset (the outer loop will attempt to read more tokens).
    }
}
