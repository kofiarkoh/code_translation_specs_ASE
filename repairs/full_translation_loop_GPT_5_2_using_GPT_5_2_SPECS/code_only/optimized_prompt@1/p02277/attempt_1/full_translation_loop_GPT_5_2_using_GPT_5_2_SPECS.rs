use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Card {
    c: u8,   // store as raw byte to mirror C char behavior closely
    val: i32,
    num: i32,
}

fn partition(a: &mut [Card], p: isize, r: isize) -> isize {
    let mut i: isize = p - 1;
    let x: i32 = a[r as usize].val;

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
    // Read entire stdin, then parse tokens similarly to scanf("%d") and scanf(" %c%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse::<usize>().unwrap(),
        None => return,
    };

    let mut a: Vec<Card> = Vec::with_capacity(n);
    for i in 0..n {
        let c_tok = it.next().unwrap();
        let c = c_tok.as_bytes()[0]; // scanf(" %c") reads a single non-whitespace char
        let val: i32 = it.next().unwrap().parse().unwrap();
        a.push(Card {
            c,
            val,
            num: i as i32,
        });
    }

    if n > 0 {
        quicksort(&mut a, 0, (n as isize) - 1);
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

    if flag == 1 {
        print!("Not stable\n");
    } else {
        print!("Stable\n");
    }

    for card in &a {
        // C prints "%c %d\n"
        print!("{} {}\n", card.c as char, card.val);
    }
}
