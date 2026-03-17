use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Card {
    character: char,
    value: i32,
}

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of cards
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the cards
    let mut cards: Vec<Card> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let character = parts[0].chars().next().unwrap();
            let value: i32 = parts[1].parse().unwrap();
            cards.push(Card { character, value });
        }
    }

    // Sort the cards using a stable sort (to preserve order of equal values)
    let mut sorted_cards = cards.clone();
    sorted_cards.sort_by(|a, b| a.value.cmp(&b.value));

    // Check stability: compare original and sorted cards
    let is_stable = check_stability(&cards, &sorted_cards);

    // Output stability result
    if is_stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    // Output sorted cards
    for card in sorted_cards {
        println!("{} {}", card.character, card.value);
    }
}

// Function to check if the sorting is stable
fn check_stability(original: &[Card], sorted: &[Card]) -> bool {
    let mut original_indices: Vec<(i32, usize)> = original
        .iter()
        .enumerate()
        .map(|(i, card)| (card.value, i))
        .collect();

    // Sort original indices based on the card values
    original_indices.sort_by(|a, b| a.0.cmp(&b.0));

    // Check if the order of indices in the sorted array matches the original
    for (i, card) in sorted.iter().enumerate() {
        let original_index = original_indices.iter()
            .position(|&(value, index)| value == card.value && index == original[i].value as usize)
            .unwrap();
        if original_index != i {
            return false; // Found a mismatch in order
        }
    }
    true
}
