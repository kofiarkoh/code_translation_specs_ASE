fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let test_cases: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..test_cases {
        // Read the sequence of card values
        let cards = lines.next().unwrap().unwrap();
        let remaining = process_cards(&cards);
        println!("{}", remaining);
    }
}

// Function to process the card sequence and return the number of remaining cards
fn process_cards(cards: &str) -> usize {
    // Ensure we have exactly 20 characters
    assert_eq!(cards.len(), 20);

    let mut stack: Vec<char> = Vec::new();

    for card in cards.chars() {
        // Check for adjacency and matching pairs
        if let Some(&last) = stack.last() {
            if last == card {
                stack.pop(); // Remove the matching pair
            } else {
                stack.push(card); // Add the current card to the stack
            }
        } else {
            stack.push(card); // Stack is empty, just add the card
        }
    }

    // The remaining cards are those left in the stack
    stack.len()
}
