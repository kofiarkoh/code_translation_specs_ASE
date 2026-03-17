use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read the card set, which should be exactly 20 characters
        let buf: String = lines.next().unwrap().unwrap();
        assert_eq!(buf.len(), 20); // Ensure the input is exactly 20 characters

        // Process the card set to find the number of remaining cards
        let remaining_cards = process_cards(&buf);
        println!("{}", remaining_cards);
    }
}

// Function to process the card set and return the number of remaining cards
fn process_cards(buf: &str) -> usize {
    let mut cards = buf.chars().collect::<Vec<_>>();
    let mut pairs_removed = true;

    // Continue removing pairs while possible
    while pairs_removed {
        pairs_removed = false;
        let mut i = 0;

        // Check for pairs in the card set
        while i < cards.len() {
            if i + 1 < cards.len() && cards[i] == cards[i + 1] {
                // Remove the pair by replacing with a placeholder
                cards.remove(i);
                cards.remove(i);
                pairs_removed = true;
                // Reset index to check from the start again
                i = 0;
            } else {
                i += 1;
            }
        }
    }

    // Return the number of remaining cards
    cards.len()
}
