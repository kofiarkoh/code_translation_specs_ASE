use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let m: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // If no input, C would likely do nothing meaningful; exit quietly.
    };
    let n: i32 = it.next().unwrap().parse().unwrap();

    // C globals: int N,M,A[106],i,j,l,r,a=0;
    // Use i32 to match typical C int behavior; use wrapping ops to emulate overflow if it occurs.
    let mut a: i32 = 0;
    let mut arr = [0i32; 106];

    for i in 0..m {
        for j in 0..n {
            let mut v: i32 = it.next().unwrap().parse().unwrap();
            if i != 0 {
                v ^= 1;
            }
            arr[j as usize] = v;
        }

        // for(l=0,r=N-1;l<=r;)
        let mut l: i32 = 0;
        let mut r: i32 = n.wrapping_sub(1);

        while l <= r {
            let lv = arr[l as usize];
            if lv == 1 {
                a = a.wrapping_add(1);
                l = l.wrapping_add(1);
                // Note: In the original C, this can access A[N] when l was N-1.
                // The array is sized 106, so as long as N <= 105 this is in-bounds.
                arr[l as usize] ^= 1;
            } else {
                let rv = arr[r as usize];
                if rv == 1 {
                    a = a.wrapping_add(1);
                    r = r.wrapping_sub(1);
                    arr[r as usize] ^= 1;
                } else {
                    l = l.wrapping_add(1);
                    arr[l as usize] ^= 1;
                }
            }
        }
    }

    print!("{}\n", a);
}
