use std::io::{self, BufRead};

fn is_straight(cards: &[i32; 5]) -> bool {
    for i in 0..4 {
        if cards[i] + 1 != cards[i + 1] {
            break;
        }
        if i == 3 {
            return true; // All cards are in a straight
        }
    }
    // Check for the special case of A, 10, J, Q, K
    if cards == &[1, 10, 11, 12, 13] {
        return true;
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let yakuname = [
        "null", "one pair", "two pair", "three card",
        "straight", "full house", "four card"
    ];

    loop {
        let mut input = String::new();
        if stdin.lock().read_line(&mut input).is_err() {
            break; // Exit on read error
        }

        let cards: Vec<i32> = input.trim()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        if cards.len() != 5 {
            break; // Exit if input does not contain exactly five integers
        }

        let mut cards_array = [0; 5];
        cards_array.copy_from_slice(&cards);

        // Sort the cards in non-decreasing order
        cards_array.sort();

        let mut yaku = 0;

        if cards_array[1] == cards_array[2] && cards_array[2] == cards_array[3] {
            if cards_array[0] == cards_array[1] || cards_array[3] == cards_array[4] {
                yaku = 6; // Full house
            } else {
                yaku = 3; // Three of a kind
            }
        } else if cards_array[0] == cards_array[1] && cards_array[1] == cards_array[2] {
            if cards_array[3] == cards_array[4] {
                yaku = 5; // Full house
            } else {
                yaku = 3; // Three of a kind
            }
        } else if cards_array[2] == cards_array[3] && cards_array[3] == cards_array[4] {
            if cards_array[0] == cards_array[1] {
                yaku = 5; // Full house
            } else {
                yaku = 3; // Three of a kind
            }
        } else if is_straight(&cards_array) {
            yaku = 4; // Straight
        } else {
            for i in 0..4 {
                if cards_array[i] == cards_array[i + 1] {
                    yaku += 1; // Count pairs
                }
            }
        }

        println!("{}", yakuname[yaku]);
    }
}
