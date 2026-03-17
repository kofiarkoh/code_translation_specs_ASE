fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let test_cases: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..test_cases {
        // Read the card sequence for each test case
        let cards: String = lines.next().unwrap().unwrap();
        
        // Ensure the input is exactly 20 characters long
        if cards.len() != 20 {
            // Behavior is not defined for invalid input, so we skip processing
            continue;
        }

        // Process the card sequence to find the number of remaining cards
        let remaining = process_cards(&cards);
        println!("{}", remaining);
    }
}

// Function to process the card sequence and return the number of remaining cards
fn process_cards(cards: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();

    for card in cards.chars() {
        // Check for adjacent matching cards
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
