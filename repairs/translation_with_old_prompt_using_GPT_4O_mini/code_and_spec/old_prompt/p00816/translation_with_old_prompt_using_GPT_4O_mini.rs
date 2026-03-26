use std::io::{self, Write};

fn func(s: &mut Vec<char>, n: usize, m: usize) -> i32 {
    let mut sum = 0;
    let mut p = 0;

    for i in 0..n {
        if m & (1 << i) != 0 {
            // Temporarily null-terminate the string
            let t = s[i + 1];
            s[i + 1] = '\0';
            // Convert the substring to a number
            let num_str: String = s[p..=i].iter().collect();
            sum += num_str.parse::<i32>().unwrap_or(0); // Handle parse error gracefully
            s[i + 1] = t;
            p = i + 1;
        }
    }
    // Convert the last segment to a number
    let last_num_str: String = s[p..].iter().collect();
    sum + last_num_str.parse::<i32>().unwrap_or(0) // Handle parse error gracefully
}

fn main() {
    let mut input = String::new();
    let mut paper = vec!['\0'; 10];

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let target: i32 = parts.next().unwrap().parse().unwrap();
        if target == 0 {
            break;
        }

        let paper_str = parts.next().unwrap();
        let n = paper_str.len();
        for (i, c) in paper_str.chars().enumerate() {
            paper[i] = c;
        }

        let mut res = 0;
        let mut best = -1;
        let mut cnt = 0;

        for i in (1 << (n - 1))..(1 << n) {
            let t = func(&mut paper, n, i);
            if t <= target {
                if best < t {
                    best = t;
                    res = i;
                    cnt = 1;
                } else if best == t {
                    cnt += 1;
                }
            }
        }

        if cnt != 1 {
            if cnt == 0 {
                println!("error");
            } else {
                println!("rejected");
            }
        } else {
            print!("{} ", best);
            for i in 0..n {
                print!("{}", paper[i]);
                if res & (1 << i) != 0 {
                    if i == n - 1 {
                        println!();
                    } else {
                        print!(" ");
                    }
                }
            }
        }
    }
}
