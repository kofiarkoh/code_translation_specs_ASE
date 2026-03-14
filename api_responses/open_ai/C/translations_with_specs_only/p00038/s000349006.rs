use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Continuously read input until an error occurs
    for line in handle.lines() {
        match line {
            Ok(input) => {
                // Parse the input and evaluate the hand
                if let Some(hand_type) = evaluate_hand(&input) {
                    println!("{}", hand_type);
                }
            }
            Err(_) => {
                // Terminate without output on read error
                return;
            }
        }
    }
}

fn evaluate_hand(input: &str) -> Option<&'static str> {
    // Split the input by commas and parse into integers
    let mut ranks: Vec<i32> = input
        .split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect();

    // Check if we have exactly five ranks
    if ranks.len() != 5 {
        return None; // Invalid input format, return None
    }

    // Sort the ranks in non-decreasing order
    ranks.sort_unstable();

    // Count occurrences of each rank
    let mut counts = [0; 14]; // Index 0 will be unused, ranks 1-13
    for &rank in &ranks {
        if rank < 1 || rank > 13 {
            return None; // Invalid rank, return None
        }
        counts[rank as usize] += 1;
    }

    // Determine the hand type based on counts
    let mut pairs = 0;
    let mut three_of_a_kind = false;
    let mut four_of_a_kind = false;

    for &count in &counts {
        if count == 2 {
            pairs += 1;
        } else if count == 3 {
            three_of_a_kind = true;
        } else if count == 4 {
            four_of_a_kind = true;
        }
    }

    // Check for straight (consecutive ranks)
    let is_straight = ranks.windows(2).all(|w| w[1] - w[0] == 1);

    // Determine the hand type
    match (four_of_a_kind, three_of_a_kind, pairs, is_straight) {
        (true, _, _, _) => Some("four card"),
        (_, true, _, _) if pairs == 1 => Some("full house"),
        (_, true, _, _) => Some("three card"),
        (_, _, 2, _) => Some("two pair"),
        (_, _, 1, _) => Some("one pair"),
        (_, _, 0, true) => Some("straight"),
        _ => Some("null"),
    }
}
