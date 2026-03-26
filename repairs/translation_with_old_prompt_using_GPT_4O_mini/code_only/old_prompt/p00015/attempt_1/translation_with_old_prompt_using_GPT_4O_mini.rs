use std::io::{self, BufRead};

const LEN: usize = 128;

fn convert(char_num: &str, int_num: &mut [i32; LEN]) {
    let len = char_num.len();
    for (j, c) in char_num.chars().rev().enumerate() {
        int_num[j] = c as i32 - 48; // Convert char to int
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let data_set: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut count = 1;

    while count <= data_set {
        count += 1;

        let first_num = lines.next().unwrap().unwrap();
        let second_num = lines.next().unwrap().unwrap();

        if first_num.len() > 80 || second_num.len() > 80 {
            println!("overflow");
            continue;
        }

        let mut f_num = [0; LEN];
        let mut s_num = [0; LEN];
        let mut sum = [0; LEN];

        convert(&first_num, &mut f_num);
        convert(&second_num, &mut s_num);

        for i in 0..LEN {
            sum[i] = f_num[i] + s_num[i];
        }

        for i in 0..LEN {
            if sum[i] >= 10 {
                sum[i] %= 10;
                if i + 1 < LEN {
                    sum[i + 1] += 1;
                }
            }
        }

        let mut k = 0;
        let mut j = LEN - 1;
        while j >= 0 {
            if sum[j] == 0 && j != 0 {
                k += 1;
                if j == 0 { break; } // Prevent underflow
                j -= 1;
            } else {
                break;
            }
        }

        if k < LEN - 80 {
            print!("overflow");
        } else {
            for l in (LEN - k - 1..=0).rev() {
                print!("{}", sum[l]);
            }
        }
        println!();
    }
}
