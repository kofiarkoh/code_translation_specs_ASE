use std::io::{self, Write};

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
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
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
    let mut d_list: Vec<List> = vec![List { next: None, v: 0, l: 0, r: d }; (n + 1) as usize];
    hash[0] = Some(Box::new(d_list[0].clone()));

    for i in 1..=n {
        let l = (x * 2 + d * (i - 1)) * i / 2;
        let r = (y * 2 - d * (i - 1)) * i / 2 + d;
        let v = (l % d + d) % d;
        let h = (v % (HASH as i64)) as usize;

        let mut p = hash[h].as_mut();
        while let Some(ref mut node) = p {
            if node.v == v && node.l <= r && node.r >= l {
                node.l = node.l.min(l);
                node.r = node.r.max(r);
                break;
            }
            p = node.next.as_mut();
        }

        if p.is_none() {
            d_list[i as usize].v = v;
            d_list[i as usize].l = l;
            d_list[i as usize].r = r;
            d_list[i as usize].next = hash[h].take();
            hash[h] = Some(Box::new(d_list[i as usize].clone()));
        }
    }

    let mut ans = 0;
    for h in 0..HASH {
        let mut p = hash[h].as_ref();
        while let Some(ref node) = p {
            ans += (node.r - node.l) / d;
            p = node.next.as_ref();
        }
    }

    println!("{}", ans);
    io::stdout().flush().unwrap();
}
