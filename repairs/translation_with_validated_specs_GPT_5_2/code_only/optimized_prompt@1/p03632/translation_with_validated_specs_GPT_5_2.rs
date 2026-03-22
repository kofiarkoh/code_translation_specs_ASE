use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse 4 integers like scanf("%d %d %d %d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a_in: i32 = it.next().unwrap().parse().unwrap();
    let b_in: i32 = it.next().unwrap().parse().unwrap();
    let c_in: i32 = it.next().unwrap().parse().unwrap();
    let d_in: i32 = it.next().unwrap().parse().unwrap();

    // Arrays sized to match the C code.
    // Note: The C code has `int c[100]` but then writes c[0..=100], which is out-of-bounds (UB).
    // To preserve the *intended* behavior (count overlaps for indices 0..=100), we size it to 101.
    let mut a = [0i32; 101];
    let mut b = [0i32; 101];
    let mut c = [0i32; 101];

    let mut time: i32 = 0;

    // The C code will go out-of-bounds if inputs are outside 0..=100; that's UB.
    // We emulate the intended behavior for in-range inputs by clamping indices to 0..=100.
    // This avoids panics while keeping behavior sensible for typical problem constraints.
    let clamp = |x: i32| -> i32 { x.max(0).min(100) };

    let a_start = clamp(a_in);
    let a_end = clamp(b_in);
    if a_start <= a_end {
        for i in a_start..=a_end {
            a[i as usize] = 1;
        }
    }

    let b_start = clamp(c_in);
    let b_end = clamp(d_in);
    if b_start <= b_end {
        for i in b_start..=b_end {
            b[i as usize] = 1;
        }
    }

    for i in 0..=100 {
        c[i] = a[i] + b[i];
        if c[i] == 2 {
            time += 1;
        }
    }

    if time == 0 {
        print!("{}", time);
    } else {
        print!("{}", time - 1);
    }
}
