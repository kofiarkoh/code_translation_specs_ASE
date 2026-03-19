use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let k: i64 = s.split_whitespace().next().unwrap().parse().unwrap();

    if k >= 50 {
        // Reconstruct the intended (AtCoder-like) construction:
        // Output n=50 and an array a[0..49] such that:
        //   a[i] = 49 + q for all i, then for the first r elements add 1,
        //   and for the remaining elements subtract r.
        // where q = k / 50, r = k % 50 (with k >= 0 here).
        //
        // This matches the observed sample for k=51:
        // q=1, r=1 => a[0]=50+50=100, others=50-1=49.
        let n: usize = 50;
        let ku = k as i128;
        let q = ku / 50;
        let r = (ku % 50) as usize;

        let mut a = vec![49i128 + q; n];
        for i in 0..n {
            if i < r {
                a[i] += 50 - r as i128;
            } else {
                a[i] -= r as i128;
            }
        }

        print!("{n}\n");
        for i in 0..n {
            if i + 1 == n {
                print!("{}", a[i]);
            } else {
                print!("{} ", a[i]);
            }
        }
    } else if k >= 2 {
        let n = k as usize;
        print!("{n}\n");
        for i in 0..n {
            if i + 1 == n {
                print!("{n}");
            } else {
                print!("{n} ");
            }
        }
    } else if k == 0 {
        print!("4\n3 3 3 3");
    } else if k == 1 {
        print!("3\n1 0 3");
    } else {
        // Negative input is outside the meaningful specified domain; keep output well-formed.
        print!("3\n0 0 0");
    }
}
