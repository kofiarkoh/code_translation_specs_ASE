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
        
        // Ensure we have exactly 20 characters
        if cards.len() != 20 {
            // Behavior is not defined for incorrect input format
            continue; // In a real scenario, we might want to handle this differently
        }

        // Calculate remaining cards after maximum removals
        let remaining = remove_pairs(&cards);
        println!("{}", remaining);
    }
}

// Function to remove pairs of adjacent matching cards
fn remove_pairs(cards: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();

    for card in cards.chars() {
        // If the stack is not empty and the top card matches the current card, pop the stack
        if let Some(&top) = stack.last() {
            if top == card {
                stack.pop(); // Remove the matching pair
            } else {
                stack.push(card); // Add the current card to the stack
            }
        } else {
            stack.push(card); // Stack is empty, push the current card
        }
    }

    // The remaining cards are those left in the stack
    stack.len()
}
