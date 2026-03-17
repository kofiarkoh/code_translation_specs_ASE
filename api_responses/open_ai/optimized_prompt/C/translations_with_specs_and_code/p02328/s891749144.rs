use std::io::{self, BufRead};

const MAX: usize = 100001;

#[derive(Debug)]
struct Rect {
    height: i64,
    pos: usize,
}

fn large_histogram(w: usize, table: &[i64]) -> i64 {
    let mut stack: Vec<Rect> = Vec::new();
    let mut max_area = 0;
    let mut extended_table = table.to_vec();
    extended_table.push(0); // Append a zero height to process remaining bars

    for i in 0..=w {
        let r = Rect {
            height: extended_table[i],
            pos: i,
        };

        if stack.is_empty() || stack.last().unwrap().height < r.height {
            stack.push(r);
        } else if stack.last().unwrap().height > r.height {
            let mut target = i;
            while !stack.is_empty() && stack.last().unwrap().height >= r.height {
                let pre = stack.pop().unwrap();
                let area = pre.height * (i as i64 - pre.pos as i64);
                if max_area < area {
                    max_area = area;
                }
                target = pre.pos;
            }
            let mut new_r = r;
            new_r.pos = target;
            stack.push(new_r);
        }
    }
    max_area
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of histogram bars
    let w: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut table = vec![0; w];

    // Read the heights of the histogram bars
    for i in 0..w {
        table[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    // Calculate the maximum area
    let max_area = large_histogram(w, &table);
    
    // Print the result
    println!("{}", max_area);
}
```

### Notes:
1. **Data Types**: The C code uses `long long int`, which is equivalent to `i64` in Rust. The maximum size of the histogram is handled by using a vector of `i64`.
2. **Stack Management**: The stack is implemented using a `Vec<Rect>`, which is idiomatic in Rust and automatically manages memory.
3. **Input Handling**: The input is read using `io::stdin()` and `BufRead`, which is a safe and efficient way to handle input in Rust.
4. **Error Handling**: The code assumes valid input as per the specification. If the input is invalid (e.g., negative number of bars), the behavior is not defined, as specified.
5. **Output**: The output format is preserved exactly as in the C code, ensuring that the newline is included after printing the maximum area.
