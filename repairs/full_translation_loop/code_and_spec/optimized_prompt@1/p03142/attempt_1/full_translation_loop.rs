use std::io::{self, BufRead};

const MAX_NODES: usize = 114514;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read N and M
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap_or_else(|_| {
        std::process::exit(1);
    });
    let m: usize = parts.next().unwrap().parse().unwrap_or_else(|_| {
        std::process::exit(1);
    });

    // Initialize structures
    let mut ec = vec![0; MAX_NODES];
    let mut es: Vec<Vec<usize>> = vec![Vec::new(); MAX_NODES];
    let mut sansyo = vec![0; MAX_NODES];
    let mut next_count = [0; 2];
    let mut next_buffer = vec![vec![0; MAX_NODES]; 2];
    let mut answer = vec![0; MAX_NODES];

    // Read edges
    for _ in 0..(n - 1 + m) {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap_or_else(|_| {
            std::process::exit(1);
        });
        let b: usize = parts.next().unwrap().parse().unwrap_or_else(|_| {
            std::process::exit(1);
        });
        ae(a, b, &mut ec, &mut es);
        sansyo[b] += 1;
    }

    // Initialize the first phase
    next_count[0] = 0;
    for i in 1..=n {
        if sansyo[i] == 0 {
            next_buffer[0][next_count[0]] = i;
            next_count[0] += 1;
        }
    }

    // Process the graph
    let mut phase = 0;
    while next_count[phase] > 0 {
        next_count[1 - phase] = 0;
        for i in 0..next_count[phase] {
            let target = next_buffer[phase][i];
            for &neighbor in &es[target] {
                if sansyo[neighbor] > 0 {
                    sansyo[neighbor] -= 1;
                    if sansyo[neighbor] == 0 {
                        answer[neighbor] = target;
                        next_buffer[1 - phase][next_count[1 - phase]] = neighbor;
                        next_count[1 - phase] += 1;
                    }
                }
            }
        }
        phase = 1 - phase;
    }

    // Output the results
    for i in 1..=n {
        println!("{}", answer[i]);
    }
}

// Function to add an edge
fn ae(f: usize, t: usize, ec: &mut Vec<usize>, es: &mut Vec<Vec<usize>>) {
    if ec[f] >= es[f].len() {
        es[f].push(t);
    } else {
        es[f][ec[f]] = t;
    }
    ec[f] += 1;
}
```

### Notes:
1. **Memory Management**: The original C code uses `realloc`, which can lead to memory allocation failures. In Rust, we use `Vec`, which handles memory safely and automatically.
2. **Error Handling**: The Rust code checks for input parsing errors and exits with status code 1 if the input format is invalid, as specified.
3. **Graph Representation**: The adjacency list is represented using a `Vec<Vec<usize>>`, which is idiomatic in Rust.
4. **Output**: The output format is preserved exactly as specified, with each parent node printed on a new line.
5. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees.
