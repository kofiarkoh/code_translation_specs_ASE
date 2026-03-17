use std::io::{self, BufRead};

const MAX_STRINGS: usize = 1024;
const MAX_LENGTH: usize = 80;

static mut A: [[u8; MAX_LENGTH]; MAX_STRINGS] = [[0; MAX_LENGTH]; MAX_STRINGS];
static mut AP: usize = 0;

fn reverse(s: &mut [u8], n: usize) {
    let half = n / 2;
    for i in 0..half {
        s.swap(i, n - 1 - i);
    }
}

fn swap(s1: &mut [u8], s2: &mut [u8]) {
    let mut t = Vec::new();
    t.extend_from_slice(s2);
    t.extend_from_slice(s1);
    let len = t.len();
    s1.copy_from_slice(&t[..len]);
}

fn contain(s: &[u8]) -> bool {
    unsafe {
        for i in 0..AP {
            if A[i] == s {
                return true;
            }
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n {
        let mut s = lines.next().unwrap().unwrap().into_bytes();
        let len = s.len();
        let t = s.clone();

        unsafe {
            AP = 0;
        }

        for i in 1..len {
            unsafe {
                if !contain(&s) {
                    A[AP].copy_from_slice(&s);
                    AP += 1;
                }
                reverse(&mut s, i);
                if !contain(&s) {
                    A[AP].copy_from_slice(&s);
                    AP += 1;
                }
                reverse(&mut s[i..], len - i);
                if !contain(&s) {
                    A[AP].copy_from_slice(&s);
                    AP += 1;
                }
                reverse(&mut s, i);
                if !contain(&s) {
                    A[AP].copy_from_slice(&s);
                    AP += 1;
                }
                s.copy_from_slice(&t);
                swap(&mut s, &mut s[i..]);
                if !contain(&s) {
                    A[AP].copy_from_slice(&s);
                    AP += 1;
                }
                reverse(&mut s[len - i..], i);
                if !contain(&s) {
                    A[AP].copy_from_slice(&s);
                    AP += 1;
                }
                reverse(&mut s, len - i);
                if !contain(&s) {
                    A[AP].copy_from_slice(&s);
                    AP += 1;
                }
                s.copy_from_slice(&t);
            }
        }
        println!("{}", unsafe { AP });
    }
}
```

### Notes:
1. **Unsafe Code**: The use of `unsafe` is necessary for manipulating the static mutable array `A` and the counter `AP`. This is because Rust's safety guarantees do not allow mutable static variables without `unsafe` blocks.
2. **String Handling**: The C code uses `char` arrays, which are translated to `u8` arrays in Rust. The `Vec<u8>` is used for temporary storage during the `swap` function.
3. **Contain Function**: The `contain` function checks if a string is already in the global array `A`, similar to the C implementation.
4. **Input Handling**: The program reads input using `BufRead` for efficiency, especially when handling multiple lines.
5. **Output**: The output format is preserved to match the specification, ensuring that each count of unique permutations is printed on a new line.
