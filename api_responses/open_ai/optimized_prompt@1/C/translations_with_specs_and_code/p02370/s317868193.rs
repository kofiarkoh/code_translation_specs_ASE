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
    let mut memo: Vec<usize> = Vec::with_capacity(e * 2);
    for _ in 0..e {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let s: usize = edge_parts.next().unwrap().parse().unwrap();
        let t: usize = edge_parts.next().unwrap().parse().unwrap();
        memo.push(s);
        memo.push(t);
        hi[s] += 1;
    }

    // Allocate adjacency list
    for i in 0..v {
        if hi[i] > 0 {
            to[i] = vec![0; hi[i]];
        }
    }

    // Reset hi for use in adjacency list population
    hi.fill(0);
    let mut j = 0;
    while j < memo.len() {
        let s = memo[j];
        let t = memo[j + 1];
        to[s][hi[s]] = t;
        hi[s] += 1;
        count[t] += 1;
        j += 2;
    }

    // Perform topological sort
    topological_sort(v, &to, &count);
}

fn topological_sort(v: usize, to: &Vec<Vec<usize>>, count: &Vec<usize>) {
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut top = 0;

    // Initialize queue with vertices that have no incoming edges
    for i in 0..v {
        if count[i] == 0 {
            q.push_back(i);
        }
    }

    while let Some(i) = q.pop_front() {
        println!("{}", i);
        top += 1;

        for &k in &to[i] {
            let new_count = count[k] - 1;
            if new_count == 0 {
                q.push_back(k);
            }
        }
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin` to read input, which is idiomatic and safe. The input parsing is done using `split_whitespace()` and `parse()`, ensuring that we handle integers correctly.
2. **Data Structures**: The adjacency list is represented as a `Vec<Vec<usize>>`, and the incoming edge counts are stored in a `Vec<usize>`, which is idiomatic in Rust.
3. **Queue for Topological Sort**: A `VecDeque` is used for the queue to efficiently pop from the front and push to the back.
4. **Memory Management**: Rust's ownership model eliminates the need for manual memory management (like `malloc` in C), making the code safer and easier to maintain.
5. **Error Handling**: The specification states that no explicit error handling is implemented, so the Rust code follows this by not adding any error checks for input validity or cycle detection.
6. **Output**: The output format is preserved by using `println!` to print each vertex on a new line.
