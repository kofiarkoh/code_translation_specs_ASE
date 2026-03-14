use std::io::{self, BufRead};

fn is_straight(cards: &[i32; 5]) -> bool {
    for i in 0..4 {
        if cards[i] + 1 != cards[i + 1] {
            break;
        }
        if i == 3 {
            return true; // All cards are consecutive
        }
    }
    // Check for the special case of Ace to 10 straight
    if cards == &[1, 10, 11, 12, 13] {
        return true;
    }
    false
}

fn main() {
    let yakuname = [
        "null", "one pair", "two pair", "three card",
        "straight", "full house", "four card",
    ];

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut cards: [i32; 5] = [0; 5];

        // Parse the input
        if let Err(_) = sscanf::sscanf!(&line, "{}, {}, {}, {}, {}", cards[0], cards[1], cards[2], cards[3], cards[4]) {
            break; // Break on error, similar to scanf returning -1
        }

        // Sort the cards using bubble sort
        for i in (1..5).rev() {
            for j in 0..i {
                if cards[j] > cards[j + 1] {
                    cards.swap(j, j + 1);
                }
            }
        }

        let mut yaku = 0;

        if cards[1] == cards[2] && cards[2] == cards[3] {
            if cards[0] == cards[1] || cards[3] == cards[4] {
                yaku = 6; // Four of a kind
            } else {
                yaku = 3; // Three of a kind
            }
        } else if cards[0] == cards[1] && cards[1] == cards[2] {
            if cards[3] == cards[4] {
                yaku = 5; // Full house
            } else {
                yaku = 3; // Three of a kind
            }
        } else if cards[2] == cards[3] && cards[3] == cards[4] {
            if cards[0] == cards[1] {
                yaku = 5; // Full house
            } else {
                yaku = 3; // Three of a kind
            }
        } else if is_straight(&cards) {
            yaku = 4; // Straight
        } else {
            for i in 0..4 {
                if cards[i] == cards[i + 1] {
                    yaku += 1; // Count pairs
                }
            }
        }

        println!("{}", yakuname[yaku]);
    }
}
