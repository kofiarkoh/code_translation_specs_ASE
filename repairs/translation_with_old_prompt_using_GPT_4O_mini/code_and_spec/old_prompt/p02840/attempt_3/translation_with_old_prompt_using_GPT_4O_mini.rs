use std::io::{self, BufRead};

const HASH: usize = 100003;

#[derive(Debug, Clone)]
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
    
    let n = nums.next().unwrap();
    let mut x = nums.next().unwrap();
    let d = nums.next().unwrap();

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
    let mut d = d / g;

    let y;
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
        let h = (v % d) as usize;

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
            d_list.push(List { next: hash[h].take(), v, l, r });
            hash[h] = Some(Box::new(d_list.last().unwrap().clone()));
        } else {
            let node = p.as_ref().unwrap();
            // Create a mutable reference to the node to modify its fields
            let mut node_mut = node.clone(); // Clone to modify
            node_mut.l = node_mut.l.min(l);
            node_mut.r = node_mut.r.max(r);
            // Update the hash table with the modified node
            hash[h] = Some(Box::new(node_mut));
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
