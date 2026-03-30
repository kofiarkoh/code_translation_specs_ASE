use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Card {
    c: char,
    val: i32,
    num: usize,
}

fn quicksort(cards: &mut [Card], p: usize, r: usize) {
    if p < r {
        let q = partition(cards, p, r);
        quicksort(cards, p, q.wrapping_sub(1)); // Use wrapping_sub to handle underflow
        quicksort(cards, q + 1, r);
    }
}

fn partition(cards: &mut [Card], p: usize, r: usize) -> usize {
    let x = cards[r].val;
    let mut i = p.wrapping_sub(1); // Use wrapping_sub to handle underflow

    for j in p..r {
        if cards[j].val <= x {
            i = i.wrapping_add(1); // Use wrapping_add to handle overflow
            cards.swap(i, j);
        }
    }
    cards.swap(i.wrapping_add(1), r); // Use wrapping_add to handle overflow
    i.wrapping_add(1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut cards = Vec::with_capacity(n);
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let c: char = parts.next().unwrap().chars().next().unwrap();
        let val: i32 = parts.next().unwrap().parse().unwrap();
        cards.push(Card { c, val, num: i });
    }

    quicksort(&mut cards, 0, n.wrapping_sub(1)); // Use wrapping_sub to handle underflow

    let mut flag = false;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if cards[i].val == cards[j].val && cards[i].num > cards[j].num {
                flag = true;
                break;
            }
        }
        if flag {
            break;
        }
    }

    if flag {
        println!("Not stable");
    } else {
        println!("Stable");
    }

    for card in cards {
        println!("{} {}", card.c, card.val);
    }
}
