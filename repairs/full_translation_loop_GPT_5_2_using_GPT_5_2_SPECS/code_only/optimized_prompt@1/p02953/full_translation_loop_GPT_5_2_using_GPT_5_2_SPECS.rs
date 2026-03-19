use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like C's "%d".
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // If no input, C would have undefined behavior; we just exit.
    };

    // C has a fixed-size array a[100009]; we mirror that.
    let mut a = vec![0i32; 100_009];

    // for(i=0;i<n;i++) scanf("%d",&a[i]);
    // If n is negative, the loop doesn't run in C; same here.
    for i in 0..(n.max(0) as usize) {
        if let Some(s) = it.next() {
            a[i] = s.parse::<i32>().unwrap();
        } else {
            // Missing inputs: C would read garbage/leave uninitialized; we keep default 0.
            break;
        }
    }

    // for(i=n-1;i>=1;i--) { ... }
    // In C, if n <= 0, i starts negative and the loop doesn't execute.
    // If n == 0, i = -1; if n == 1, i = 0; both skip due to i>=1.
    let mut i = n - 1;
    while i >= 1 {
        let idx = i as usize;

        if a[idx] < a[idx - 1] {
            // C's a[i-1]-- on int can overflow (UB). Use wrapping to emulate typical behavior.
            a[idx - 1] = a[idx - 1].wrapping_sub(1);
        }
        if a[idx] < a[idx - 1] {
            print!("No");
            return;
        }

        i -= 1;
    }

    print!("Yes");
}
