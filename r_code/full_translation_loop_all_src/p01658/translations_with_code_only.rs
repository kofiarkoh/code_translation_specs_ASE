use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let m: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let n: i32 = it.next().unwrap().parse().unwrap();

    // C globals: int A[106], a=0; indices i,j,l,r are int.
    // We'll keep A as i32 to match C int behavior for comparisons and XOR with 1.
    let mut a: i32 = 0;
    let mut arr = vec![0i32; 106];

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
        let mut r: i32 = n - 1;

        while l <= r {
            let lv = arr[l as usize];
            if lv == 1 {
                a = a.wrapping_add(1);
                // A[++l] ^= 1; (note: in C this can go out of bounds when l==r==N-1)
                l = l.wrapping_add(1);
                let idx = l as usize;
                arr[idx] ^= 1;
            } else {
                let rv = arr[r as usize];
                if rv == 1 {
                    a = a.wrapping_add(1);
                    // A[--r] ^= 1; (can go out of bounds when r==0)
                    r = r.wrapping_sub(1);
                    let idx = r as usize;
                    arr[idx] ^= 1;
                } else {
                    // A[++l] ^= 1; (can go out of bounds when l==N-1)
                    l = l.wrapping_add(1);
                    let idx = l as usize;
                    arr[idx] ^= 1;
                }
            }
        }
    }

    print!("{}\n", a);
}
