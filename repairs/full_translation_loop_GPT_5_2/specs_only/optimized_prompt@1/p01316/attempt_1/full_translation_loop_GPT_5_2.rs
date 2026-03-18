use std::io::{self, Read};

const INF: i64 = 1_000_000_000_000_000_000; // predefined large constant for "no valid transformation"

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break, // malformed input => unspecified; stop
        };
        let m: usize = match it.next() {
            Some(s) => match s.parse() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };

        // Read m adjustment values.
        let mut adj: Vec<i64> = Vec::with_capacity(m);
        for _ in 0..m {
            if let Some(s) = it.next() {
                adj.push(s.parse::<i64>().unwrap());
            } else {
                // malformed input => unspecified; stop
                return;
            }
        }

        // Read n input values.
        let mut vals: Vec<i64> = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(s) = it.next() {
                vals.push(s.parse::<i64>().unwrap());
            } else {
                // malformed input => unspecified; stop
                return;
            }
        }

        // Specification is ambiguous about:
        // - the exact transformation rule,
        // - the allowed range for transformed values,
        // - the cost function,
        // - the DP state definition and matrix size.
        //
        // Narrowest reasonable interpretation that preserves stated invariants:
        // - We must output a non-negative minimum cost, or INF if no valid transformation exists.
        // - With no concrete constraints/range/cost definition, we cannot derive any valid
        //   transformation criteria; therefore, we conservatively treat the instance as having
        //   no provably valid transformation unless it is vacuously valid.
        //
        // Vacuous case: if n == 0, there are no values to transform, so minimum cost is 0.
        // Otherwise, output INF.
        let ans: i64 = if n == 0 { 0 } else { INF };

        // Note: We still "apply" adjustment values consistently by reading them, but without
        // a defined rule, we cannot compute a meaningful DP.
        let _ = adj;
        let _ = vals;

        outputs.push(ans.to_string());
    }

    print!("{}", outputs.join("\n"));
}
