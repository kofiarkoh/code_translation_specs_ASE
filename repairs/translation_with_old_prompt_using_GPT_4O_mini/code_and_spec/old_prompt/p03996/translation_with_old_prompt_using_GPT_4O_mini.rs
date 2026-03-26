use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Data {
    prev: Option<Box<Data>>,
    next: Option<Box<Data>>,
    id: usize,
    count: usize,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read n, m, q
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let mut params = input.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = params.next().unwrap();
    let m = params.next().unwrap();
    let q = params.next().unwrap();

    // Read operations
    let mut a = vec![0; q + 1];
    for i in 1..=q {
        input.clear();
        handle.read_line(&mut input).unwrap();
        a[i] = input.trim().parse::<usize>().unwrap();
    }

    // Prepare data structures
    let mut d: Vec<Data> = (0..=m).map(|i| Data {
        prev: None,
        next: None,
        id: i,
        count: n,
    }).collect();

    let mut p: Option<&mut Data> = None;
    d[0].count = n;

    // Core logic
    for i in (1..=q).rev() {
        if d[a[i]].count == 0 {
            if let Some(prev) = p {
                d[a[i]].prev = Some(Box::new(prev.clone()));
                prev.next = Some(Box::new(d[a[i]].clone()));
            }
            p = Some(&mut d[a[i]]);
        }
        d[a[i]].count = d[a[i]].count.min(d[d[a[i]].prev.as_ref().map_or(0, |prev| prev.id)].count);
    }

    let mut complete = vec![false; m + 1];
    let mut allow = 0;
    let mut current = d.iter().find(|data| data.count == n);

    while let Some(data) = current {
        complete[data.id] = true;
        while allow < complete.len() && complete[allow] {
            allow += 1;
        }
        current = data.next.as_ref().and_then(|next| next.as_ref());
    }

    while let Some(data) = current {
        if data.id != allow {
            println!("No");
            return;
        }
        complete[allow] = true;
        while allow < complete.len() && complete[allow] {
            allow += 1;
        }
        current = data.next.as_ref().and_then(|next| next.as_ref());
    }

    println!("Yes");
}
