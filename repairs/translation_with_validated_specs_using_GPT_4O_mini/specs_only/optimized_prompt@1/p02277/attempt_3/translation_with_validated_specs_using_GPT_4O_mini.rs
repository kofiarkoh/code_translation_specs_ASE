use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Card {
    character: char,
    value: i32,
    original_index: usize, // To check stability
}

fn quicksort(cards: &mut [Card]) {
    if cards.len() <= 1 {
        return;
    }
    let pivot_index = partition(cards);
    quicksort(&mut cards[0..pivot_index]);
    quicksort(&mut cards[pivot_index + 1..]);
}

fn partition(cards: &mut [Card]) -> usize {
    let pivot = cards[cards.len() - 1].value;
    let mut i = 0;
    for j in 0..cards.len() - 1 {
        if cards[j].value < pivot {
            cards.swap(i, j);
            i += 1;
        }
    }
    cards.swap(i, cards.len() - 1);
    i
}

fn is_stable(original: &[Card], sorted: &[Card]) -> bool {
    let mut last_seen: HashMap<i32, Vec<usize>> = HashMap::new();
    
    // Record original indices of cards with the same value
    for (index, card) in original.iter().enumerate() {
        last_seen.entry(card.value).or_insert_with(Vec::new).push(index);
    }

    // Check if the sorted order maintains the original indices
    for card in sorted {
        if let Some(indices) = last_seen.get_mut(&card.value) {
            if indices.is_empty() || indices[0] != card.original_index {
                return false; // Found a card that is not in original order
            }
            indices.remove(0); // Remove the used index
        }
    }
    true
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut cards = Vec::with_capacity(n);

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let character = parts[0].chars().next().unwrap();
        let value: i32 = parts[1].parse().unwrap();
        cards.push(Card { character, value, original_index: i });
    }

    // Sort the cards using quicksort
    let mut sorted_cards = cards.clone();
    quicksort(&mut sorted_cards);

    // Check stability
    let stable = is_stable(&cards, &sorted_cards);

    // Output results
    println!("{}", if stable { "Stable" } else { "Not stable" });
    for card in sorted_cards {
        println!("{} {}", card.character, card.value);
    }
}
