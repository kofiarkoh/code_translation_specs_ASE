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
        b
    } else {
        gcd(b % a, a)
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let (n, mut x, mut d) = (nums[0], nums[1], nums[2]);

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

    let y = if d < 0 {
        let temp = x;
        x += d * (n - 1);
        d *= -1;
        temp
    } else {
        x + d * (n - 1)
    };

    let mut hash: Vec<Option<Box<List>>> = vec![None; HASH];
    let mut d_list: Vec<List> = vec![List { next: None, v: 0, l: 0, r: d }; (n + 1) as usize];
    hash[0] = Some(Box::new(d_list[0].clone()));

    for i in 1..=n {
        let l = (x * 2 + d * (i - 1)) * i / 2;
        let r = (y * 2 - d * (i - 1)) * i / 2 + d;
        let v = (l % d + d) % d;
        let h = (v % HASH) as usize;

        let mut p = &hash[h];
        let mut found = false;

        while let Some(ref node) = p {
            if node.v == v && node.l <= r && node.r >= l {
                found = true;
                break;
            }
            p = &node.next;
        }

        if !found {
            d_list[i].v = v;
            d_list[i].l = l;
            d_list[i].r = r;
            d_list[i].next = hash[h].take();
            hash[h] = Some(Box::new(d_list[i].clone()));
        } else {
            let mut node = p.as_ref().unwrap();
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
}
