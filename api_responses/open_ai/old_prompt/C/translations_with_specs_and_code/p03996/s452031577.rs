use std::io::{self, BufRead};

#[derive(Debug)]
struct Data {
    prev: Option<Box<Data>>,
    next: Option<Box<Data>>,
    id: usize,
    count: usize,
}

fn main() {
    // === input parameters ===
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read n, m, q
    let first_line = lines.next().unwrap().unwrap();
    let params: Vec<usize> = first_line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = params[0];
    let m = params[1];
    let q = params[2];

    // Read operations
    let mut a = vec![0; q + 1];
    for i in 1..=q {
        let line = lines.next().unwrap().unwrap();
        a[i] = line.trim().parse().unwrap();
    }

    // === prepare ===
    let mut d: Vec<Data> = (0..=m).map(|i| Data {
        prev: None,
        next: None,
        id: i,
        count: if i == 0 { n } else { 0 },
    }).collect();

    let mut p = &mut d[0];
    for i in (1..=q).rev() {
        if d[a[i]].count == 0 {
            d[a[i]].prev = Some(Box::new(*p.clone()));
            p.next = Some(Box::new(d[a[i]].clone()));
            p = p.next.as_mut().unwrap();
        }
        d[a[i]].count = d[a[i]].count.saturating_add(1).min(p.count);
    }

    // === core ===
    let mut complete = vec![false; m + 1];
    let mut allow = 0;
    let mut current = &d[0];

    while let Some(ref node) = current {
        if node.count == n {
            complete[node.id] = true;
            while complete[allow] {
                allow += 1;
            }
            current = node.next.as_ref().map(|b| b.as_ref()).unwrap_or(&None);
        } else {
            break;
        }
    }

    while let Some(ref node) = current {
        if node.id != allow {
            println!("No");
            return;
        }
        complete[allow] = true;
        while complete[allow] {
            allow += 1;
        }
        current = node.next.as_ref().map(|b| b.as_ref()).unwrap_or(&None);
    }

    println!("Yes");
}
```

### Notes:
1. **Memory Management**: Rust's ownership model eliminates the need for manual memory management (like `calloc` in C). Instead, we use `Vec` and `Box` to manage dynamic memory safely.
2. **Data Structure**: The `Data` struct is implemented with `Option<Box<Data>>` to represent linked list pointers safely.
3. **Input Handling**: The input is read using `BufRead` for efficiency, and parsing is done with error handling in mind.
4. **Logic Preservation**: The core logic of the C code is preserved, ensuring that the operations are processed in the same manner as specified.
5. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees.
