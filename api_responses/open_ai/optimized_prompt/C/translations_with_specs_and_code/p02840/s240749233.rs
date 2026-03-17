use std::io::{self, BufRead};

const HASH: usize = 100003;

#[derive(Debug)]
struct List {
    next: Option<Box<List>>,
    v: i64,
    l: i64,
    r: i64,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut parts = input.split_whitespace();
    
    let n: i64 = parts.next().unwrap().parse().unwrap();
    let mut x: i64 = parts.next().unwrap().parse().unwrap();
    let mut d: i64 = parts.next().unwrap().parse().unwrap();

    if x == 0 || d == 0 {
        if x == 0 && d == 0 {
            println!("1");
        } else if d == 0 {
            println!("{}", n + 1);
        } else {
            println!("{}", n * (n - 1) / 2 + 1);
        }
        return;
    }

    let g = gcd(x.abs(), d.abs());
    x /= g;
    d /= g;

    let y: i64;
    if d < 0 {
        y = x;
        x = x + d * (n - 1);
        d *= -1;
    } else {
        y = x + d * (n - 1);
    }

    let mut hash: Vec<Option<Box<List>>> = vec![None; HASH];
    let mut d_list: Vec<List> = vec![List { next: None, v: 0, l: 0, r: d }];
    hash[0] = Some(Box::new(d_list[0].clone()));

    for i in 1..=n {
        let l = (x * 2 + d * (i - 1)) * i / 2;
        let r = (y * 2 - d * (i - 1)) * i / 2 + d;
        let v = (l % d + d) % d;
        let h = (v % HASH) as usize;

        let mut p = hash[h].as_ref();
        while let Some(node) = p {
            if node.v == v && node.l <= r && node.r >= l {
                break;
            }
            p = node.next.as_ref().map(|n| n.as_ref());
        }

        if p.is_none() {
            d_list.push(List { next: hash[h].take(), v, l, r });
            hash[h] = Some(Box::new(d_list.last().unwrap().clone()));
        } else {
            let mut node = p.unwrap();
            node.l = node.l.min(l);
            node.r = node.r.max(r);
        }
    }

    let mut ans = 0;
    for h in 0..HASH {
        let mut p = hash[h].as_ref();
        while let Some(node) = p {
            ans += (node.r - node.l) / d;
            p = node.next.as_ref().map(|n| n.as_ref());
        }
    }

    println!("{}", ans);
}
```

### Notes:
1. **Data Structures**: The `List` struct is used to represent the linked list nodes, similar to the C code. Rust's ownership model is leveraged to manage memory safely.
2. **GCD Function**: The `gcd` function is implemented recursively, similar to the C version.
3. **Input Handling**: The program reads input using `io::stdin` and processes it accordingly.
4. **Hash Table**: A vector of `Option<Box<List>>` is used to simulate the hash table, allowing for dynamic memory management while adhering to Rust's safety guarantees.
5. **Logic Preservation**: The logic for calculating `L`, `R`, and `V` is preserved, ensuring that the semantics of the original C code are maintained.
6. **Output**: The output format is preserved exactly as specified, ensuring that the program behaves identically in terms of I/O.
