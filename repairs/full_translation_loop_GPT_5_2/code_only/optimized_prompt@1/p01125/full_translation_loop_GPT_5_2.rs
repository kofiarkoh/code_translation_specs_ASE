use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse like scanf would (whitespace-separated tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Direction deltas as in C: d4[]={0,1,0,-1,0};
    let d4: [i64; 5] = [0, 1, 0, -1, 0];

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let mut n: i64 = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        // C: ll a[25][25]={};
        // Use fixed-size array; indices are assumed valid per C code behavior.
        let mut a = [[0i64; 25]; 25];

        for _ in 0..n {
            let x: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;
            let y: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;
            a[x][y] = 1;
        }

        let m: i64 = it.next().unwrap().parse().unwrap();

        let mut px: i64 = 10;
        let mut py: i64 = 10;

        // If starting cell has an item, collect it.
        if a[px as usize][py as usize] != 0 {
            a[px as usize][py as usize] = 0;
            n -= 1;
        }

        for _ in 0..m {
            let c_str = it.next().unwrap();
            let c = c_str.as_bytes()[0] as char;
            let mut q: i64 = it.next().unwrap().parse().unwrap();

            let d: usize = if c == 'N' {
                0
            } else if c == 'E' {
                1
            } else if c == 'S' {
                2
            } else {
                3
            };

            while q > 0 {
                q -= 1;
                px += d4[d];
                py += d4[d + 1];

                let ux = px as usize;
                let uy = py as usize;
                if a[ux][uy] != 0 {
                    a[ux][uy] = 0;
                    n -= 1;
                }
            }
        }

        if n != 0 {
            print!("No\n");
        } else {
            print!("Yes\n");
        }
    }
}
