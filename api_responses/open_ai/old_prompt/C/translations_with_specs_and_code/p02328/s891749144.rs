use std::io::{self, BufRead};

const MAX: usize = 100001;

#[derive(Clone, Copy)]
struct Rect {
    height: i64,
    pos: i64,
}

fn large_histogram(w: usize, table: &mut [i64; MAX]) -> i64 {
    let mut sp = 0;
    let mut max_area = 0;
    table[w] = 0; // Set the sentinel value

    for i in 0..=w {
        let mut r = Rect {
            height: table[i],
            pos: i as i64,
        };

        if sp == 0 || stack[sp - 1].height < r.height {
            stack[sp] = r;
            sp += 1;
        } else if stack[sp - 1].height > r.height {
            let mut target = i as i64;
            while sp > 0 && stack[sp - 1].height >= r.height {
                let pre = stack[sp - 1];
                sp -= 1; // Pop the stack
                let area = pre.height * (i as i64 - pre.pos);
                if max_area < area {
                    max_area = area;
                }
                target = pre.pos;
            }
            r.pos = target;
            stack[sp] = r;
            sp += 1;
        }
    }
    max_area
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let w: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut table: [i64; MAX] = [0; MAX];

    for i in 0..w {
        table[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    let max_area = large_histogram(w, &mut table);
    println!("{}", max_area);
}
```

### Notes:
1. **Data Types**: The C code uses `long long int`, which corresponds to `i64` in Rust.
2. **Array Size**: The `table` array is defined with a fixed size of `MAX`, which is consistent with the C code.
3. **Stack Management**: The stack is managed using a simple array and a size counter (`sp`), similar to the C implementation.
4. **Input Handling**: The input is read using `io::stdin()` and processed line by line, ensuring that the input format is preserved.
5. **Output**: The output is printed in the same format as specified, ensuring that it matches the expected behavior.
6. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees.
