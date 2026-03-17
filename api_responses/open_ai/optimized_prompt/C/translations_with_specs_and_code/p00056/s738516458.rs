use std::io::{self, BufRead};

const MAX: usize = 50001;

fn make_prime_table() -> Vec<u8> {
    let mut table = vec![0; MAX];
    
    for i in 2..MAX {
        if table[i] == 0 {
            for j in 2..(MAX / i) {
                table[i * j] = 1; // Marking non-prime numbers
            }
        }
    }
    table
}

fn count_prime_pairs(n: usize, ptable: &[u8]) -> usize {
    let mut count = 0;
    for i in 2..=n / 2 {
        if ptable[i] == 0 && ptable[n - i] == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let ptable = make_prime_table();
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let n: usize = match line {
            Ok(line) => line.trim().parse().unwrap_or(0),
            Err(_) => continue, // Skip invalid lines
        };

        if n == 0 {
            break;
        } else {
            let count = count_prime_pairs(n, &ptable);
            println!("{}", count);
        }
    }
}
```

### Explanation of Translation Choices:
1. **Prime Table Generation**: The `make_prime_table` function initializes a vector of size `MAX` and marks non-prime numbers similarly to the C code. The inner loop condition is adjusted to avoid out-of-bounds access.
  
2. **Counting Prime Pairs**: The `count_prime_pairs` function counts distinct pairs of primes that sum to `n`, preserving the logic from the C code.

3. **Input Handling**: The program reads from standard input using `io::stdin()` and processes each line. It safely handles potential parsing errors by using `unwrap_or(0)` to default to zero if parsing fails.

4. **Output**: The output format is preserved exactly as specified, ensuring that each count is printed on a new line.

5. **Safety**: The Rust code avoids `unsafe` blocks entirely, adhering to the requirement for safe, idiomatic Rust. 

6. **Error Handling**: The program assumes valid input as per the specification, so it does not handle invalid types or out-of-bounds values explicitly.
