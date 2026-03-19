use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut digit = vec![-1; n];
    let d = [100, 10, 1];
    let mut num = 0;

    if m == 0 {
        match n {
            1 => num = 0,
            2 => num = 10,
            3 => num = 100,
            _ => {}
        }
    } else {
        let mut s = vec![0; m];
        let mut c = vec![0; m];

        for i in 0..m {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut pair_iter = input.split_whitespace();
            s[i] = pair_iter.next().unwrap().parse::<usize>().unwrap();
            c[i] = pair_iter.next().unwrap().parse::<i32>().unwrap();

            let index = s[i] - 1; // Convert to 0-based index
            if digit[index] == -1 {
                digit[index] = c[i] * d[3 - n + index];
            } else if digit[index] / d[3 - n + index] != c[i] {
                num = -1;
            }
        }

        if num != -1 {
            for i in 0..n {
                if digit[i] != -1 {
                    num += digit[i];
                } else if n > 1 && i == 0 && digit[i] != 0 {
                    num += d[3 - n + i];
                }
            }
        }
    }

    if n > 1 && s[0] == 1 && c[0] == 0 {
        println!("-1");
    } else {
        println!("{}", num);
    }
}
