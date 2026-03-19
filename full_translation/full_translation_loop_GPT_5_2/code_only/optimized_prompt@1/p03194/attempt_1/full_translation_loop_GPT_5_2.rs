use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as C scanf("%ld%ld") would (whitespace-separated).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In the C code, n and p are `long` (typically 64-bit on modern judges).
    // If input is missing/malformed, C's behavior would be undefined; we unwrap.
    let n: i64 = it.next().unwrap().parse().unwrap();
    let mut p: i64 = it.next().unwrap().parse().unwrap();

    let mut ans: i64 = 1;

    // for(long i=2;i*i<=p;i++)
    // Use i128 for the i*i comparison to avoid overflow in Rust while matching intent.
    let mut i: i64 = 2;
    while (i as i128) * (i as i128) <= (p as i128) {
        let mut cnt: i64 = 0;
        // while(p%i<1)cnt++,p/=i;
        // In C, p%i<1 is equivalent to p%i==0 for positive p and i.
        // We preserve the exact condition.
        while p % i < 1 {
            cnt += 1;
            p /= i;
        }
        if cnt >= n {
            // for(int j=0;j<cnt/n;j++)ans*=i;
            // C uses signed overflow UB; we use wrapping_mul to emulate typical wraparound.
            let times = cnt / n;
            for _j in 0..(times as i32) {
                ans = ans.wrapping_mul(i);
            }
        }
        i += 1;
    }

    // if(p>1&&n==1)ans*=p;
    if p > 1 && n == 1 {
        ans = ans.wrapping_mul(p);
    }

    // printf("%ld\n",ans);
    print!("{ans}\n");
}
