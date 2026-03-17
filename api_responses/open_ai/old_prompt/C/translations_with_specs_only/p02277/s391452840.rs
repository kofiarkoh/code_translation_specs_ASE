use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Card {
    c: char,
    val: i32,
    original_index: usize, // To track the original index for stability check
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of cards
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input for number of cards");
    
    // Ensure n is greater than 0
    if n == 0 {
        panic!("n must be greater than 0");
    }

    let mut cards: Vec<Card> = Vec::with_capacity(n);

    // Read the cards
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let c: char = parts.next().unwrap().chars().next().unwrap();
        let val: i32 = parts.next().unwrap().parse().expect("Invalid input for card value");
        cards.push(Card { c, val, original_index: i });
    }

    // Sort the cards using quicksort
    quicksort(&mut cards, 0, n as isize - 1);

    // Check for stability
    let is_stable = check_stability(&cards);

    // Output the stability status
    if is_stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    // Output the sorted cards
    for card in cards {
        println!("{} {}", card.c, card.val);
    }
}

// Quicksort implementation
fn quicksort(cards: &mut Vec<Card>, low: isize, high: isize) {
    if low < high {
        let p = partition(cards, low, high);
        quicksort(cards, low, p - 1);
        quicksort(cards, p + 1, high);
    }
}

fn partition(cards: &mut Vec<Card>, low: isize, high: isize) -> isize {
    let pivot = cards[high as usize].val;
    let mut i = low - 1;

    for j in low..high {
        if cards[j as usize].val <= pivot {
            i += 1;
            cards.swap(i as usize, j as usize);
        }
    }
    cards.swap((i + 1) as usize, high as usize);
    i + 1
}

// Check if the sorting is stable
fn check_stability(cards: &[Card]) -> bool {
    let mut last_val = None;
    let mut last_index = None;

    for card in cards {
        if let Some(val) = last_val {
            if card.val == val && last_index.unwrap() > card.original_index {
                return false; // Found a case where stability is violated
            }
        }
        last_val = Some(card.val);
        last_index = Some(card.original_index);
    }
    true
}
