use std::io::{self, BufRead};

#[derive(Debug)]
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
```

### Notes:
1. **Data Structure**: The `Data` struct is implemented using `Option<Box<Data>>` for the linked list structure, allowing for safe memory management without using `unsafe`.
2. **Input Handling**: The input is read using `BufRead` for efficiency, especially for larger inputs.
3. **Logic Translation**: The core logic is preserved, ensuring that the operations are processed in the same manner as in the C code.
4. **Output**: The output format is maintained as specified, printing "Yes" or "No" based on the conditions checked.
5. **Memory Management**: Rust's ownership model is leveraged to manage memory safely, avoiding manual memory allocation and deallocation.
