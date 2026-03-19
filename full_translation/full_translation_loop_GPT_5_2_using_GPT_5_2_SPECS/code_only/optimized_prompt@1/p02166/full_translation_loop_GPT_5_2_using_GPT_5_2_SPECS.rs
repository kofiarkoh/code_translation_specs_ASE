use std::io::{self, Read};

fn dfs(a: &mut [i32], n: usize) {
    let mut t = a[n] as usize;
    while a[t] != -1 {
        let temp = a[t] as usize;
        a[t] = -1;
        t = temp;
    }
}

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse::<i32>().unwrap() as usize,
        None => return,
    };
    let k: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;

    // C uses 1-based indexing and accesses a[1..=n], with a[0] unused.
    // Values are used as indices too; assume input respects that as in the C code.
    let mut a = vec![0i32; 100010];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse::<i32>().unwrap();
    }

    if n == k {
        // int x=a[1]-1;
        let x = a[1] - 1;
        let mut flag: i32 = 1;
        for i in 1..=n {
            // (a[i]-i+n)%n == x, with C int arithmetic.
            // Use i64 to avoid Rust overflow panics; C int overflow is UB, but typical inputs won't hit it.
            let ai = a[i] as i64;
            let ii = i as i64;
            let nn = n as i64;
            let expr = ((ai - ii + nn) % nn) as i32;
            flag &= if expr == x { 1 } else { 0 };
        }
        if flag != 0 {
            print!("Yes\n");
        } else {
            print!("No\n");
        }
    } else {
        let mut cnt: i32 = 0;
        for i in 1..=n {
            if a[i] != -1 {
                dfs(&mut a, i);
                cnt += 1;
            }
        }
        // puts((n-cnt)%2&&k%2?"No":"Yes");
        let n_minus_cnt = (n as i32) - cnt;
        let cond = (n_minus_cnt % 2 != 0) && ((k as i32) % 2 != 0);
        if cond {
            print!("No\n");
        } else {
            print!("Yes\n");
        }
    }
}
