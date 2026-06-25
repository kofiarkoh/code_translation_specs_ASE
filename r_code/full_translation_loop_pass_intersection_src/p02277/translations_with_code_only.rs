use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Card {
    c: u8,   // store as raw byte to mirror C char behavior closely
    val: i32,
    num: i32,
}

fn partition(a: &mut [Card], p: i32, r: i32) -> i32 {
    let mut i = p - 1;
    let x = a[r as usize].val;

    for j in p..r {
        if a[j as usize].val <= x {
            i += 1;
            a.swap(i as usize, j as usize);
        }
    }
    a.swap((i + 1) as usize, r as usize);
    i + 1
}

fn quicksort(a: &mut [Card], p: i32, r: i32) {
    if p < r {
        let q = partition(a, p, r);
        quicksort(a, p, q - 1);
        quicksort(a, q + 1, r);
    }
}

fn main() {
    // Read entire stdin and parse as whitespace-separated tokens.
    // This matches scanf("%d") and scanf(" %c%d") behavior (leading whitespace skipped).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<Card> = Vec::with_capacity(n);
    for i in 0..n {
        let c_tok = it.next().unwrap_or("");
        let c = c_tok.as_bytes().get(0).copied().unwrap_or(b'\0');
        let val: i32 = it.next().unwrap().parse().unwrap();
        a.push(Card {
            c,
            val,
            num: i as i32,
        });
    }

    if n > 0 {
        quicksort(&mut a, 0, (n as i32) - 1);
    }

    let mut flag = 0;
    'outer: for i in 0..n.saturating_sub(1) {
        for j in (i + 1)..n {
            if a[i].val == a[j].val && a[i].num > a[j].num {
                flag = 1;
                break 'outer;
            }
        }
    }

    let mut out = String::new();
    if flag == 1 {
        out.push_str("Not stable\n");
    } else {
        out.push_str("Stable\n");
    }
    for card in &a {
        out.push(card.c as char);
        out.push(' ');
        out.push_str(&format!("{}\n", card.val));
    }

    print!("{out}");
}
