use std::io::{self, Read};

#[derive(Clone)]
struct Card {
    ch: char,
    val: i32,
    idx: usize, // original position to check stability
}

fn partition(a: &mut [Card], p: isize, r: isize) -> isize {
    let x = a[r as usize].val;
    let mut i = p - 1;
    for j in p..r {
        if a[j as usize].val <= x {
            i += 1;
            a.swap(i as usize, j as usize);
        }
    }
    a.swap((i + 1) as usize, r as usize);
    i + 1
}

fn quicksort(a: &mut [Card], p: isize, r: isize) {
    if p < r {
        let q = partition(a, p, r);
        quicksort(a, p, q - 1);
        quicksort(a, q + 1, r);
    }
}

fn main() {
    // Input format is assumed valid per specification; no explicit error handling.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut cards: Vec<Card> = Vec::with_capacity(n);

    for idx in 0..n {
        let ch_str = it.next().unwrap();
        let ch = ch_str.chars().next().unwrap();
        let val: i32 = it.next().unwrap().parse().unwrap();
        cards.push(Card { ch, val, idx });
    }

    // Use a quicksort-like algorithm (as implied by the spec) and then check stability.
    let mut sorted = cards.clone();
    if n > 0 {
        quicksort(&mut sorted, 0, (n as isize) - 1);
    }

    // Stability check: for equal values, original indices must be non-decreasing.
    let mut stable = true;
    for i in 1..n {
        if sorted[i - 1].val == sorted[i].val && sorted[i - 1].idx > sorted[i].idx {
            stable = false;
            break;
        }
    }

    if stable {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    for c in sorted {
        println!("{} {}", c.ch, c.val);
    }
}
