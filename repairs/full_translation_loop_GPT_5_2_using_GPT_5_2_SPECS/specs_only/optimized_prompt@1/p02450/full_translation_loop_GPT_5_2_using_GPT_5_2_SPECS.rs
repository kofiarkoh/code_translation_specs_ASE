use std::io::{self, Read};

fn main() {
    // Read entire stdin; no explicit error handling per specification.
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    // Parse a single integer. If parsing fails, behavior is unspecified by the spec;
    // we choose a narrow, compilable behavior: do nothing.
    let n_opt = input.split_whitespace().next().and_then(|s| s.parse::<i64>().ok());
    let Some(n_i64) = n_opt else {
        return;
    };

    // Spec precondition: non-negative. For negative, behavior is not inferable;
    // choose narrow behavior: do nothing.
    if n_i64 < 0 {
        return;
    }

    let n = n_i64 as usize;

    // If n == 0, print exactly one empty line.
    if n == 0 {
        println!();
        return;
    }

    // Generate permutations of 1..=n and print each on its own line.
    // Use backtracking with a used[] bitmap; stack usage proportional to n.
    let mut perm: Vec<usize> = Vec::with_capacity(n);
    let mut used: Vec<bool> = vec![false; n + 1]; // indices 1..=n used

    fn backtrack(n: usize, perm: &mut Vec<usize>, used: &mut [bool]) {
        if perm.len() == n {
            // Print space-separated integers on one line.
            for (i, v) in perm.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", v);
            }
            println!();
            return;
        }

        for v in 1..=n {
            if !used[v] {
                used[v] = true;
                perm.push(v);
                backtrack(n, perm, used);
                perm.pop();
                used[v] = false;
            }
        }
    }

    backtrack(n, &mut perm, &mut used);
}
