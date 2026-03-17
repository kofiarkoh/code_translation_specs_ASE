use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Card {
    c: char,
    val: i32,
    num: usize,
}

fn quicksort(cards: &mut [Card], p: usize, r: usize) {
    if p < r {
        let q = partition(cards, p, r);
        quicksort(cards, p, q.wrapping_sub(1));
        quicksort(cards, q + 1, r);
    }
}

fn partition(cards: &mut [Card], p: usize, r: usize) -> usize {
    let x = cards[r].val;
    let mut i = p.wrapping_sub(1);

    for j in p..r {
        if cards[j].val <= x {
            i += 1;
            cards.swap(i, j);
        }
    }
    cards.swap(i + 1, r);
    i + 1
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut cards = Vec::with_capacity(n);

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let c: char = parts.next().unwrap().chars().next().unwrap();
        let val: i32 = parts.next().unwrap().parse().unwrap();
        cards.push(Card { c, val, num: i });
    }

    quicksort(&mut cards, 0, n.wrapping_sub(1));

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

    if stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    for card in cards {
        println!("{} {}", card.c, card.val);
    }
}
