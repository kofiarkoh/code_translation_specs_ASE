fn main() {
    use std::io::{self, BufRead};

    // Read the number of rounds from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get the number of rounds
    let num_rounds: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Process each round
    for _ in 0..num_rounds {
        let mut score = 0;
        let mut outs = 0;

        // Read exactly three events for each round
        for _ in 0..3 {
            let event = lines.next().unwrap().unwrap();

            match event.as_str() {
                "HIT" => {
                    // A hit increases the score by 1
                    score += 1;
                }
                "HOMERUN" => {
                    // A home run increases the score by 4
                    score += 4;
                }
                "OUT" => {
                    // An out increases the out count
                    outs += 1;
                }
                _ => {
                    // If an unexpected event occurs, we ignore it
                    // This is a safeguard; the specification does not define behavior for invalid inputs
                }
            }

            // Ensure the number of outs does not exceed 3
            if outs > 3 {
                outs = 3; // Cap outs at 3, as per the invariant
            }
        }

        // Output the score for the current round
        println!("{}", score);
    }
}
