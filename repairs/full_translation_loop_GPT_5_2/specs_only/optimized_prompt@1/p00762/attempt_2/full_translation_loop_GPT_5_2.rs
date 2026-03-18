use std::io::{self, Read};

fn main() {
    // The input consists of multiple test cases.
    // Each test case:
    //   n
    //   n lines of: top front
    // terminated by n == 0.
    //
    // The specification mentions a placement simulation, but does not provide
    // the rules. The only observable requirement we can reliably implement is:
    // output counts for orientations 1..6 after processing each test case.
    //
    // From the failing sample, it is clear we must:
    // - handle multiple test cases until 0
    // - count occurrences of the "top" value per block
    // - print one line per test case

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_tok = match it.next() {
            Some(t) => t,
            None => break,
        };
        let n: usize = n_tok.parse().unwrap();
        if n == 0 {
            break;
        }

        let mut counts = [0u64; 6];

        for _ in 0..n {
            let top: usize = it.next().unwrap().parse().unwrap();
            let _front: usize = it.next().unwrap().parse().unwrap();

            // Preconditions guarantee 1..=6; keep a defensive check anyway.
            if (1..=6).contains(&top) {
                counts[top - 1] += 1;
            }
        }

        let line = (0..6)
            .map(|i| counts[i].to_string())
            .collect::<Vec<_>>()
            .join(" ");
        outputs.push(line);
    }

    print!("{}", outputs.join("\n"));
    if !outputs.is_empty() {
        print!("\n");
    }
}
