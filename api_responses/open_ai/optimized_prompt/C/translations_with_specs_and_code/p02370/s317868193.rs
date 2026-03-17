use std::collections::VecDeque;
use std::io::{self, BufRead};

const MAX_V: usize = 10002;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of vertices and edges
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let v: usize = parts.next().unwrap().parse().unwrap();
    let e: usize = parts.next().unwrap().parse().unwrap();

    let mut to: Vec<Vec<usize>> = vec![Vec::new(); v];
    let mut hi: Vec<usize> = vec![0; v];
    let mut count: Vec<usize> = vec![0; v];

    // Read edges
    let mut memo: Vec<(usize, usize)> = Vec::with_capacity(e);
    for _ in 0..e {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let s: usize = edge_parts.next().unwrap().parse().unwrap();
        let t: usize = edge_parts.next().unwrap().parse().unwrap();
        memo.push((s, t));
        hi[s] += 1;
    }

    // Initialize adjacency list
    for (s, t) in memo {
        to[s].push(t);
        count[t] += 1;
    }

    // Perform topological sort
    topological_sort(v, &to, &count, &hi);
}

fn topological_sort(v: usize, to: &Vec<Vec<usize>>, count: &Vec<usize>, hi: &Vec<usize>) {
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut local_count = count.clone(); // Local copy to maintain incoming edge counts

    // Initialize queue with vertices that have no incoming edges
    for i in 0..v {
        if local_count[i] == 0 {
            q.push_back(i);
        }
    }

    while let Some(i) = q.pop_front() {
        println!("{}", i);
        for &k in &to[i] {
            if let Some(c) = local_count.get_mut(k) {
                *c -= 1;
                if *c == 0 {
                    q.push_back(k);
                }
            }
        }
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin` to read input, which is idiomatic and safe. The input parsing is done using `split_whitespace()` and `parse()`, ensuring that we handle integers correctly.
2. **Data Structures**: The adjacency list is represented as a `Vec<Vec<usize>>`, and the incoming edge counts are stored in a `Vec<usize>`, similar to the C code.
3. **Topological Sort Logic**: The logic for topological sorting is preserved, using a `VecDeque` for efficient queue operations.
4. **Memory Management**: Rust's ownership model eliminates the need for manual memory management (like `malloc` in C), making the code safer and less error-prone.
5. **Error Handling**: The specification states that no explicit error handling is implemented, so the Rust code follows suit. However, it is inherently safer due to Rust's type system and memory safety guarantees.
6. **Output**: The output format is preserved, printing each vertex on a new line as specified.
