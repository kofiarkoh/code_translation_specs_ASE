use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 to emulate C's (likely) 32-bit int behavior
    // while still being able to represent the large constants used in R.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse::<i64>().unwrap() as usize,
        None => return,
    };

    // C has A[10000000]; we only need N elements.
    let mut a_vec: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v = it.next().unwrap().parse::<i64>().unwrap();
        a_vec.push(v);
    }

    // In the C code: int R[3]={9999999999,9999999999,9999999999};
    // This overflows 32-bit int; to preserve the *intended* behavior (a large sentinel),
    // we store it in i64.
    let mut r: [i64; 3] = [9_999_999_999, 9_999_999_999, 9_999_999_999];

    for i in 0..n {
        for j in 0..n {
            if i != j {
                // for(k=10;A[j]%k!=A[j];k*=10);
                // This loop is problematic for negative A[j] in C (can become infinite).
                // We preserve the same behavior: for negative values, the condition
                // A[j] % k != A[j] is always true for k>=10, so it would not terminate.
                // In Rust, we implement the same loop; it may also not terminate.
                let mut k: i64 = 10;
                while a_vec[j] % k != a_vec[j] {
                    k = k.wrapping_mul(10);
                }

                // a = A[i]*k + A[j]; with C int overflow. Use wrapping i64 ops to emulate.
                let mut a = a_vec[i].wrapping_mul(k).wrapping_add(a_vec[j]);

                // for(k=0;k<3;k++) if(R[k]>a) a^=R[k]^=a^=R[k];
                // The XOR-swap expression in C relies on sequencing/UB; intended effect is swap.
                for kk in 0..3 {
                    if r[kk] > a {
                        // Swap a and r[kk]
                        let tmp = r[kk];
                        r[kk] = a;
                        a = tmp;
                    }
                }
            }
        }
    }

    // printf("%d\n",R[2]); In C this prints as signed int; we print the i64 value.
    // Keep exact formatting: number + newline.
    println!("{}", r[2]);
}
