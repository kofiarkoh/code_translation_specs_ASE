use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of rounds
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        let mut score = 0;
        let mut base = [0; 3]; // Represents the bases
        let mut outs = 0;

        while outs < 3 {
            // Read the event
            let event = lines.next().unwrap().unwrap();

            match event.as_str() {
                "HIT" => {
                    base[0] += 1; // Advance the runner on first base
                    for i in 0..3 {
                        if base[i] > 1 {
                            if i < 2 {
                                base[i + 1] += 1; // Advance the runner to the next base
                            } else {
                                score += 1; // Score a run
                            }
                            base[i] -= 1; // Remove the runner from the current base
                        }
                    }
                }
                "HOMERUN" => {
                    for i in 0..3 {
                        if base[i] == 1 {
                            score += 1; // Score a run for each runner on base
                            base[i] = 0; // Clear the base
                        }
                    }
                    score += 1; // Score for the home run itself
                }
                "OUT" => {
                    outs += 1; // Increment the out count
                }
                _ => {
                    // Handle unexpected input gracefully
                    eprintln!("Unexpected event: {}", event);
                }
            }
        }
        println!("{}", score); // Output the score for the round
    }
}
```

### Notes:
- The Rust code preserves the semantics of the original C code while adhering to the provided specification.
- The input handling uses `BufRead` for efficient reading of lines, which is idiomatic in Rust.
- The event handling is done using a `match` statement, which is more idiomatic than using string comparisons.
- The code includes error handling for unexpected events, which is a good practice even if not specified in the original C code.
- The base and score calculations are directly translated from the C logic to maintain the same behavior.
