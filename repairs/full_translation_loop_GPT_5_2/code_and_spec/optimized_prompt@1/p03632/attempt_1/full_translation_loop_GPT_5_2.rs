use std::io::{self, Read};

fn main() {
    // Read all input (matches scanf behavior for whitespace-separated ints)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a_in: usize = it.next().unwrap().parse().unwrap();
    let b_in: usize = it.next().unwrap().parse().unwrap();
    let c_in: usize = it.next().unwrap().parse().unwrap();
    let d_in: usize = it.next().unwrap().parse().unwrap();

    // Arrays tracking membership in each range.
    // Note: The C code declares `c[100]` but then indexes it up to 100 inclusive,
    // which is out-of-bounds (undefined behavior). The specification requires
    // arrays sized to cover indices 0..=100, so we use length 101 here.
    let mut a = [0i32; 101];
    let mut b = [0i32; 101];
    let mut c = [0i32; 101];

    for i in a_in..=b_in {
        a[i] = 1;
    }
    for i in c_in..=d_in {
        b[i] = 1;
    }

    let mut time: i32 = 0;
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
