// AOJ 0334 Amidakuji
// Translated from C to Rust

use std::io::{self, BufRead, Write};

const MAX: usize = 1005;

fn read_input() -> (usize, Vec<Vec<usize>>) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut grid = vec![vec![0; n]; n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            if c == '1' {
                grid[j][i] = 1;
            }
        }
    }
    
    (n, grid)
}

fn bipartite_matching(n: usize, to: &mut Vec<Vec<usize>>, hi: &mut Vec<usize>, match_arr: &mut Vec<i32>) -> usize {
    let mut max = 0;
    let mut mark = 1;

    for u in 0..n {
        if match_arr[u] < 0 {
            mark += 1;
            if bpm(u, &mut mark, to, hi, match_arr) {
                max += 1;
            }
        }
    }
    max
}

fn bpm(u: usize, mark: &mut i32, to: &mut Vec<Vec<usize>>, hi: &mut Vec<usize>, match_arr: &mut Vec<i32>) -> bool {
    let mut seen = vec![0; MAX];
    seen[u] = *mark;

    for &v in &to[u][..hi[u]] {
        let w = match_arr[v];
        if w < 0 || (seen[w as usize] != *mark && bpm(w as usize, mark, to, hi, match_arr)) {
            match_arr[v] = u as i32;
            match_arr[u] = v as i32;
            return true;
        }
    }
    false
}

fn main() {
    let (n, grid) = read_input();
    let mut hi = vec![0; n];
    let mut to = vec![vec![]; n];
    let mut match_arr = vec![-1; MAX];

    for i in 0..n {
        for j in 0..n {
            if grid[j][i] == 1 {
                to[j].push(n + i);
                hi[j] += 1;
            }
        }
    }

    if bipartite_matching(n, &mut to, &mut hi, &mut match_arr) < n {
        println!("no");
    } else {
        println!("yes");
        for i in 0..n {
            match_arr[match_arr[i] as usize] = -1;
            match_arr[i] = -1;
            let mut mark = 1;
            bpm(i, &mut mark, &mut to, &mut hi, &mut match_arr);
            hi[i] = 0;
            to[i].push(match_arr[i] as usize);
            println!("{}", match_arr[i] - n + 1);
        }
    }
}
```

### Notes:
1. **Input Handling**: The input is read using `BufRead` for efficiency, similar to the C code's fast input methods.
2. **Data Structures**: The `to` and `hi` arrays are initialized as vectors of vectors to accommodate dynamic sizes.
3. **Matching Logic**: The matching logic is preserved, with the recursive function `bpm` implemented in a way that maintains the original semantics.
4. **Output**: The output format is preserved exactly as specified, ensuring that the results are printed in the correct format.
5. **Safety**: The code avoids `unsafe` Rust, adhering to idiomatic practices while ensuring that the logic remains intact.
