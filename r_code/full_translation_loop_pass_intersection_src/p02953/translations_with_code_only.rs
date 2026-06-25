use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like C's "%d".
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // If no input, C would have undefined behavior; just exit.
    };

    // C has a fixed array a[100009]; we mirror that size.
    let mut a = vec![0i32; 100_009];

    // Read n integers into a[0..n).
    for i in 0..(n as usize) {
        if let Some(s) = it.next() {
            a[i] = s.parse().unwrap();
        } else {
            // Missing input: C would read garbage/leave uninitialized; we keep 0.
            break;
        }
    }

    // for(i=n-1;i>=1;i--) with i as signed int in C.
    // If n <= 0, the loop doesn't execute in C (starts at -1, condition i>=1 false).
    let mut i = n - 1;
    while i >= 1 {
        let idx = i as usize;
        let prev = (i - 1) as usize;

        if a[idx] < a[prev] {
            // C's int decrement can overflow (UB). Use wrapping to emulate typical behavior.
            a[prev] = a[prev].wrapping_sub(1);
        }
        if a[idx] < a[prev] {
            print!("No");
            return;
        }

        i -= 1;
    }

    print!("Yes");
}
