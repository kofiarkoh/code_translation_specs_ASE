use std::io::{self, Write};

fn dfs(a: &mut [i32], n: usize) {
    let mut t = a[n];
    while a[t as usize] != -1 {
        let temp = a[t as usize];
        a[t as usize] = -1;
        t = temp;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut a = vec![0; n + 1]; // Using 1-based indexing
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    for (i, value) in input.split_whitespace().enumerate() {
        a[i + 1] = value.parse().unwrap();
    }

    if n == k as usize {
        let x = a[1] - 1;
        let mut flag = true;
        for i in 1..=n {
            flag &= (a[i] - i as i32 + n as i32) % n as i32 == x;
        }
        println!("{}", if flag { "Yes" } else { "No" });
    } else {
        let mut cnt = 0;
        for i in 1..=n {
            if a[i] != -1 {
                dfs(&mut a, i);
                cnt += 1;
            }
        }
        println!("{}", if (n - cnt) % 2 == 1 && k % 2 == 1 { "No" } else { "Yes" });
    }
}
