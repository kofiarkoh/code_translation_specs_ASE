use std::io::{self, BufRead};

const HASH: usize = 100003;

#[derive(Debug, Clone)] // Implement Clone for List
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
    let mut nums = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    let N = nums.next().unwrap();
    let X = nums.next().unwrap();
    let D = nums.next().unwrap();

    if X == 0 || D == 0 {
        if X == 0 && D == 0 {
            println!("1");
        } else if D == 0 {
            println!("{}", N + 1);
        } else {
            println!("{}", N * (N - 1) / 2 + 1);
        }
        return;
    }

    let G = gcd(X.abs(), D.abs());
    let mut X = X / G;
    let mut D = D / G;
    let Y: i64;

    if D < 0 {
        Y = X;
        X = X + D * (N - 1);
        D *= -1;
    } else {
        Y = X + D * (N - 1);
    }

    let mut hash: Vec<Option<Box<List>>> = vec![None; HASH];
    let mut d: Vec<List> = vec![List { next: None, v: 0, l: 0, r: D }; (N + 1) as usize];
    hash[0] = Some(Box::new(d[0].clone()));

    for i in 1..=N {
        let L = (X * 2 + D * (i - 1)) * i / 2;
        let R = (Y * 2 - D * (i - 1)) * i / 2 + D;
        let V = (L % D + D) % D;
        let h = (V % D) as usize;

        let mut p = &hash[h];
        let mut found = false;

        while let Some(ref node) = p {
            if node.v == V && node.l <= R && node.r >= L {
                found = true;
                break;
            }
            p = &node.next;
        }

        if !found {
            d[i].v = V;
            d[i].l = L;
            d[i].r = R;
            d[i].next = hash[h].take();
            hash[h] = Some(Box::new(d[i].clone()));
        } else {
            let mut node = p.as_ref().unwrap();
            node.l = node.l.min(L);
            node.r = node.r.max(R);
        }
    }

    let mut ans = 0;
    for h in 0..HASH {
        let mut p = &hash[h];
        while let Some(ref node) = p {
            ans += (node.r - node.l) / D;
            p = &node.next;
        }
    }

    println!("{}", ans);
}
