use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Card {
    c: char,
    val: i32,
    num: usize,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of cards
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut cards: Vec<Card> = Vec::with_capacity(n);

    // Read each card
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let c: char = parts.next().unwrap().chars().next().unwrap();
        let val: i32 = parts.next().unwrap().parse().unwrap();
        cards.push(Card { c, val, num: i });
    }

    // Sort the cards using quicksort
    quicksort(&mut cards, 0, n as isize - 1);

    // Check for stability
    let mut stable = true;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if cards[i].val == cards[j].val && cards[i].num > cards[j].num {
                stable = false;
                break;
            }
        }
        if !stable {
            break;
        }
    }

    // Output stability status
    if stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    // Output sorted cards
    for card in cards {
        println!("{} {}", card.c, card.val);
    }
}

fn quicksort(cards: &mut Vec<Card>, p: isize, r: isize) {
    if p < r {
        let q = partition(cards, p, r);
        quicksort(cards, p, q - 1);
        quicksort(cards, q + 1, r);
    }
}

fn partition(cards: &mut Vec<Card>, p: isize, r: isize) -> isize {
    let x = cards[r as usize].val;
    let mut i = p - 1;

    for j in p..r {
        if cards[j as usize].val <= x {
            i += 1;
            cards.swap(i as usize, j as usize);
        }
    }
    cards.swap((i + 1) as usize, r as usize);
    i + 1
}
