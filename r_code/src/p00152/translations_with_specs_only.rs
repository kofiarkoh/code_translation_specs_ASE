// bowling_scores.rs
//
// Translates the provided specification into a Rust program.
// - Reads datasets from stdin: each dataset starts with integer n (number of players).
// - For each player reads an integer id, then consumes as many subsequent integers
//   as needed to score exactly one ten-frame bowling game for that player.
// - Scores are computed using standard ten-pin bowling rules (strikes/spares bonuses).
// - Players are then sorted by descending score, ties broken by ascending id.
// - For each player in sorted order prints "id score" on its own line.
// The program stops when n == 0 or when input runs out while attempting to read n.

use std::io::{self, Read};

struct Scanner {
    tokens: Vec<String>,
    pos: usize,
}

impl Scanner {
    fn new_from_stdin() -> io::Result<Self> {
        let mut s = String::new();
        // Read entire stdin. This mirrors typical scanf behavior where the program
        // will block until EOF (interactive or piped). It also makes handling EOF
        // straightforward: when tokens are exhausted we treat it as end-of-input.
        io::stdin().read_to_string(&mut s)?;
        let tokens = s.split_whitespace().map(|t| t.to_string()).collect();
        Ok(Scanner { tokens, pos: 0 })
    }

    // Return next integer if available, otherwise None.
    fn next_i32(&mut self) -> Option<i32> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        match tok.parse::<i32>() {
            Ok(v) => Some(v),
            Err(_) => None, // If a token is non-numeric, treat as EOF/scan failure.
        }
    }
}

// Compute the score for a single ten-frame bowling game by consuming integers
// from the provided scanner as roll pin counts. Returns Some(score) on success,
// or None if input exhausted while trying to read required rolls.
fn scoring(scanner: &mut Scanner) -> Option<i32> {
    // We will lazily read rolls into this vector as needed.
    let mut rolls: Vec<i32> = Vec::new();

    // helper to ensure rolls.len() > idx by reading more tokens if necessary.
    // returns true if rolls has element at idx, false if unable to read (EOF).
    let mut ensure_roll = |idx: usize, scanner: &mut Scanner| -> bool {
        while rolls.len() <= idx {
            match scanner.next_i32() {
                Some(v) => rolls.push(v),
                None => return false,
            }
        }
        true
    };

    let mut score: i32 = 0;
    let mut roll_index: usize = 0;

    // Process frames 1..=10
    for frame in 0..10 {
        // Ensure at least first roll of this frame is available
        if !ensure_roll(roll_index, scanner) {
            return None;
        }
        let first = rolls[roll_index];

        if frame < 9 {
            // Frames 1-9
            if first == 10 {
                // Strike: needs next two rolls for bonus
                if !ensure_roll(roll_index + 1, scanner) {
                    return None;
                }
                if !ensure_roll(roll_index + 2, scanner) {
                    return None;
                }
                let bonus1 = rolls[roll_index + 1];
                let bonus2 = rolls[roll_index + 2];
                score += 10 + bonus1 + bonus2;
                roll_index += 1; // strike consumes one roll
            } else {
                // Two-roll frame
                if !ensure_roll(roll_index + 1, scanner) {
                    return None;
                }
                let second = rolls[roll_index + 1];
                let frame_sum = first + second;
                if frame_sum == 10 {
                    // Spare: needs next one roll for bonus
                    if !ensure_roll(roll_index + 2, scanner) {
                        return None;
                    }
                    let bonus = rolls[roll_index + 2];
                    score += 10 + bonus;
                } else {
                    // Open frame
                    score += frame_sum;
                }
                roll_index += 2;
            }
        } else {
            // 10th frame (frame == 9)
            if first == 10 {
                // Strike: needs two extra rolls
                if !ensure_roll(roll_index + 1, scanner) {
                    return None;
                }
                if !ensure_roll(roll_index + 2, scanner) {
                    return None;
                }
                let b1 = rolls[roll_index + 1];
                let b2 = rolls[roll_index + 2];
                score += 10 + b1 + b2;
                // Consumption of rolls for 10th frame isn't used further
            } else {
                // Need second roll
                if !ensure_roll(roll_index + 1, scanner) {
                    return None;
                }
                let second = rolls[roll_index + 1];
                let frame_sum = first + second;
                if frame_sum == 10 {
                    // Spare: needs one extra roll
                    if !ensure_roll(roll_index + 2, scanner) {
                        return None;
                    }
                    let bonus = rolls[roll_index + 2];
                    score += 10 + bonus;
                } else {
                    // Open frame
                    score += frame_sum;
                }
            }
            // After 10th frame we are done
            break;
        }
    }

    Some(score)
}

fn main() {
    let mut scanner = match Scanner::new_from_stdin() {
        Ok(s) => s,
        Err(_) => {
            // If we cannot read stdin for some reason, exit normally (no datasets processed).
            return;
        }
    };

    // Process datasets repeatedly
    loop {
        // Read dataset size n
        let n = match scanner.next_i32() {
            Some(v) => v,
            None => break, // EOF or non-integer: terminate
        };

        if n == 0 {
            break; // stop when n == 0
        }

        // For safety and faithfulness to spec we accept any n; spec expects n <= 40.
        // If input is malformed (not enough player records), we'll exit when reads fail.
        let mut players: Vec<(i32, i32)> = Vec::new(); // (id, score)

        // Read n player records
        for _ in 0..n {
            // Read player id
            let id = match scanner.next_i32() {
                Some(v) => v,
                None => return, // EOF while expecting id: terminate program
            };

            // Compute score for this player's single game by consuming rolls from scanner
            let sc = match scoring(&mut scanner) {
                Some(sv) => sv,
                None => return, // EOF while reading rolls: terminate program
            };

            players.push((id, sc));
        }

        // Sort players: descending score, ties by ascending id
        players.sort_by(|a, b| {
            // a and b are (id, score)
            let (id_a, sc_a) = (a.0, a.1);
            let (id_b, sc_b) = (b.0, b.1);
            // Primary: score descending
            match sc_b.cmp(&sc_a) {
                std::cmp::Ordering::Equal => id_a.cmp(&id_b), // tie-breaker: id ascending
                other => other,
            }
        });

        // Print players
        for (id, sc) in players {
            println!("{} {}", id, sc);
        }
    }

    // Program ends normally; implicit return with exit code 0.
}
