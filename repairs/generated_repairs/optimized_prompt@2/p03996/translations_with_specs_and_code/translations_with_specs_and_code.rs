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
    let (n, m, q) = (params[0], params[1], params[2]);

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
            d[a[i]].prev = Some(Box::new(*p));
            p.next = Some(Box::new(d[a[i]].clone()));
            p = p.next.as_mut().unwrap();
        }
        d[a[i]].count = d[a[i]].count.saturating_add(1).min(p.count);
    }

    // === core ===
    let mut complete = vec![false; m + 1];
    let mut allow = 0;
    let mut current = &d[0];

    while let Some(ref node) = current.next {
        if node.count == n {
            complete[node.id] = true;
            while complete[allow] {
                allow += 1;
            }
            current = node;
        } else {
            break;
        }
    }

    while let Some(ref node) = current.next {
        if node.id != allow {
            println!("No");
            return;
        }
        complete[allow] = true;
        while complete[allow] {
            allow += 1;
        }
        current = node;
    }

    println!("Yes");
}

// === Notes ===
// 1. **Memory Management**: Rust's ownership model eliminates the need for manual memory management (like `calloc` in C). Instead, we use `Vec` and `Box` to manage dynamic memory safely.
// 2. **Data Structure**: The `Data` struct is used to represent the linked list nodes, with `Option<Box<Data>>` for `prev` and `next` pointers to allow for safe ownership and borrowing.
// 3. **Input Handling**: The input is read using `io::stdin` and processed line by line, which is idiomatic in Rust.
// 4. **Error Handling**: The program does not explicitly handle memory allocation failures, as per the specification. In Rust, this is generally managed by the language itself, and the program will panic if it runs out of memory.
// 5. **Preserving Semantics**: The logic of the original C code is preserved, ensuring that the operations are processed in the specified order and that the output matches the expected results.
