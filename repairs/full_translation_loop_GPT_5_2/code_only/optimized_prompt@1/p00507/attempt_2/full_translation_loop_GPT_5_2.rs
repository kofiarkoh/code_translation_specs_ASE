use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let n_usize = if n < 0 { 0 } else { n as usize };

    // C: int A[10000000]; we only store the first N values actually read.
    let mut a_vec: Vec<i32> = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        let v = it.next().unwrap_or("0").parse::<i32>().unwrap();
        a_vec.push(v);
    }

    // C: int R[3]={9999999999,9999999999,9999999999};
    // This overflows 32-bit int; emulate typical 2's-complement wrap.
    let init_r: i32 = 9_999_999_999u64 as u32 as i32;
    let mut r: [i32; 3] = [init_r, init_r, init_r];

    for i in 0..n_usize {
        for j in 0..n_usize {
            if i != j {
                // for(k=10; A[j]%k!=A[j]; k*=10);
                // Use wrapping arithmetic to emulate C int overflow behavior.
                let mut k: i32 = 10;
                loop {
                    let aj = a_vec[j];
                    if aj.wrapping_rem(k) == aj {
                        break;
                    }
                    k = k.wrapping_mul(10);
                }

                // a = A[i]*k + A[j]; (wrapping like C int)
                let mut a: i32 = a_vec[i].wrapping_mul(k).wrapping_add(a_vec[j]);

                // for(k=0;k<3;k++) if(R[k]>a) a^=R[k]^=a^=R[k];
                for kk in 0..3 {
                    if r[kk] > a {
                        // XOR-swap; bitwise ops don't overflow, so plain ^ matches C.
                        a ^= r[kk];
                        r[kk] ^= a;
                        a ^= r[kk];
                    }
                }
            }
        }
    }

    // printf("%d\n",R[2]);
    print!("{}\n", r[2]);
}
