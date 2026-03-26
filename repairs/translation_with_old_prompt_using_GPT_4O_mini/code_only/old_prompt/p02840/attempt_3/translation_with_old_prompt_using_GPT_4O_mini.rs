use std::io::{self, Write};

const HASH: usize = 100003;

#[derive(Debug)]
struct List {
    next: Option<Box<List>>,
    v: i64,
    l: i64,
    r: i64,
}

impl List {
    fn new(v: i64, l: i64, r: i64, next: Option<Box<List>>) -> Self {
        List { next, v, l, r }
    }
}

// Implement Clone for List to allow cloning of List instances
impl Clone for List {
    fn clone(&self) -> Self {
        List {
            next: self.next.as_ref().map(|n| Box::new((*n).clone())),
            v: self.v,
            l: self.l,
            r: self.r,
        }
    }
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let mut x: i64 = iter.next().unwrap().parse().unwrap();
    let mut d: i64 = iter.next().unwrap().parse().unwrap();

    if x == 0 || d == 0 {
        if x == 0 && d == 0 {
            println!("1");
        } else if d == 0 {
            println!("{}", n + 1);
        } else {
            println!("{}", n * (n - 1) / 2 + 1);
        }
        io::stdout().flush().unwrap();
        return;
    }

    let g = gcd(x.abs(), d.abs());
    x /= g;
    d /= g;

    let y: i64;
    if d < 0 {
        y = x;
        x += d * (n - 1);
        d *= -1;
    } else {
        y = x + d * (n - 1);
    }

    let mut hash: Vec<Option<Box<List>>> = vec![None; HASH];
    let mut d_list: Vec<List> = vec![List::new(0, 0, d, None); (n + 1) as usize];
    hash[0] = Some(Box::new(d_list[0].clone()));

    for i in 1..=n {
        let l = (x * 2 + d * (i - 1)) * i / 2;
        let r = (y * 2 - d * (i - 1)) * i / 2 + d;
        let v = (l % d + d) % d;
        let h = (v % d) as usize % HASH;

        let mut p = &hash[h];
        while let Some(ref node) = p {
            if node.v == v && node.l <= r && node.r >= l {
                break;
            }
            p = &node.next;
        }

        if p.is_none() {
            d_list[i as usize] = List::new(v, l, r, hash[h].take());
            hash[h] = Some(Box::new(d_list[i as usize].clone()));
        } else {
            let node = p.as_ref().unwrap();
            node.l = node.l.min(l);
            node.r = node.r.max(r);
        }
    }

    let mut ans = 0;
    for h in 0..HASH {
        let mut p = &hash[h];
        while let Some(ref node) = p {
            ans += (node.r - node.l) / d;
            p = &node.next;
        }
    }

    println!("{}", ans);
    io::stdout().flush().unwrap();
}
