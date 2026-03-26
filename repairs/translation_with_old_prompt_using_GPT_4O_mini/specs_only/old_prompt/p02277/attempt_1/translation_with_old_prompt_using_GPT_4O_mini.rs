use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Card {
    c: char,
    val: i32,
    index: usize, // To track original index for stability check
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
    let pivot = cards[cards.len() - 1].val;
    let mut i = 0;
    for j in 0..cards.len() - 1 {
        if cards[j].val < pivot {
            cards.swap(i, j);
            i += 1;
        }
    }
    cards.swap(i, cards.len() - 1);
    i
}

fn check_stability(original: &[Card], sorted: &[Card]) -> bool {
    let mut last_val = None;
    let mut last_index = None;

    for card in sorted {
        if let Some(val) = last_val {
            if card.val == val {
                if card.index < last_index.unwrap() {
                    return false; // Not stable
                }
            }
        }
        last_val = Some(card.val);
        last_index = Some(card.index);
    }
    true // Stable
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read number of cards
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut cards: Vec<Card> = Vec::with_capacity(n);

    // Read cards
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let c = parts[0].chars().next().unwrap();
        let val: i32 = parts[1].parse().unwrap();
        cards.push(Card { c, val, index: i });
    }

    // Sort cards
    let mut sorted_cards = cards.clone();
    quicksort(&mut sorted_cards);

    // Check stability
    let stable = check_stability(&cards, &sorted_cards);

    // Output results
    if stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }
    for card in sorted_cards {
        println!("{} {}", card.c, card.val);
    }
}
