const MAX: usize = 1_000_001;
const MOD: i64 = 1_000_000_007;

static mut FAC: [i64; MAX] = [0; MAX];
static mut FINV: [i64; MAX] = [0; MAX];
static mut INV: [i64; MAX] = [0; MAX];

fn com_init() {
    unsafe {
        FAC[0] = 1;
        FAC[1] = 1;
        FINV[0] = 1;
        FINV[1] = 1;
        INV[1] = 1;
        for i in 2..MAX {
            FAC[i] = (FAC[i - 1] * i as i64) % MOD;
            INV[i] = (MOD - INV[(MOD % i) as usize] * (MOD / i as i64) % MOD) % MOD;
            FINV[i] = (FINV[i - 1] * INV[i]) % MOD;
        }
    }
}

fn com(n: i64, k: i64) -> i64 {
    if n < k || n < 0 || k < 0 {
        return 0;
    }
    unsafe {
        (FAC[n as usize] * (FINV[k as usize] * FINV[(n - k) as usize] % MOD)) % MOD
    }
}

fn main() {
    use std::io::{self, Write};

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let x: i64 = iter.next().unwrap().parse().unwrap();
    let y: i64 = iter.next().unwrap().parse().unwrap();

    if (x + y) % 3 != 0 {
        println!("0");
        return;
    }
    let n = (x + y) / 3;

    let x_ = x - n;
    let y_ = y - n;

    if x_ < 0 || y_ < 0 {
        println!("0");
        return;
    }

    com_init();
    println!("{}", com(x_ + y_, x_) % MOD);
}
``` 

### Explanation of Choices:
1. **Static Mutable Arrays**: The C code uses global arrays for factorials and inverses. In Rust, we use `static mut` to achieve similar behavior. This is necessary to maintain the same state across function calls, but it requires `unsafe` blocks to access mutable static variables.
  
2. **Integer Types**: The C code uses `long long`, which corresponds to `i64` in Rust. This ensures we can handle large numbers without overflow.

3. **Modulo Operations**: The modulo operations are preserved to ensure the same behavior as in C, especially in the context of potential negative results.

4. **Input Handling**: The input is read using `std::io`, which is idiomatic in Rust. The parsing is done with error handling to ensure robustness.

5. **Output Formatting**: The output format is preserved exactly as in the C code, ensuring that the newline and formatting match.

6. **Corner Cases**: The checks for negative values and the conditions for returning `0` are directly translated to maintain the same logic flow as in the original C code.
