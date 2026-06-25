use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: i64 = it.next().unwrap().parse().unwrap();

    // 1-based array, a[0] unused
    let mut a: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    if (n as i64) == k {
        // Check uniform cyclic shift: (a[i] - i) mod n is constant
        let x = a[1] - 1;
        let mut flag = true;
        let nn = n as i64;
        for i in 1..=n {
            let ii = i as i64;
            let v = (a[i] - ii + nn).rem_euclid(nn);
            if v != x {
                flag = false;
                break;
            }
        }
        if flag {
            print!("Yes\n");
        } else {
            print!("No\n");
        }
    } else {
        // Count components/cycles in functional graph by mutating a with sentinel -1.
        // This follows the C code's behavior: it assumes mapping values are valid in 1..=n
        // and that -1 is not a legitimate mapping value in this branch.
        fn dfs(start: usize, a: &mut [i64]) {
            let mut t = a[start] as usize;
            while a[t] != -1 {
                let temp = a[t] as usize;
                a[t] = -1;
                t = temp;
            }
        }

        let mut cnt: i64 = 0;
        for i in 1..=n {
            if a[i] != -1 {
                dfs(i, &mut a);
                cnt += 1;
            }
        }

        let n_minus_cnt_odd = ((n as i64 - cnt) % 2) != 0;
        let k_odd = (k % 2) != 0;
        if n_minus_cnt_odd && k_odd {
            print!("No\n");
        } else {
            print!("Yes\n");
        }
    }
}
