use std::io::{self, Read};

const MAX: usize = 13;

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut numline = [0i32; MAX];
    let mut tmpline = [0i32; MAX];
    let mut num = [0i32; 101];

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // scanf("%d",&n) != 1
        }
        let n: usize = n_opt.unwrap().parse::<i32>().unwrap() as usize;
        if n == 0 {
            break;
        }

        for i in 0..n {
            // C code assumes inputs fit; we mirror that and index directly.
            numline[i] = it.next().unwrap().parse::<i32>().unwrap();
        }

        let mut flag: i32 = 1;
        let mut cnt: i32 = 0;

        while flag != 0 {
            // num[i] = 0
            for v in num.iter_mut() {
                *v = 0;
            }

            // num[numline[i]]++
            for i in 0..n {
                let idx = numline[i] as usize;
                num[idx] += 1;
            }

            // tmpline[i] = num[numline[i]]
            for i in 0..n {
                let idx = numline[i] as usize;
                tmpline[i] = num[idx];
            }

            cnt += 1;
            flag = 0;

            // count positions where tmpline[i] == numline[i]
            for i in 0..n {
                if tmpline[i] == numline[i] {
                    flag += 1;
                }
            }

            if flag == n as i32 {
                flag = 0;
            } else {
                for i in 0..n {
                    numline[i] = tmpline[i];
                }
                flag = 1;
            }
        }

        out.push_str(&(cnt - 1).to_string());
        out.push('\n');
        for i in 0..n {
            if i == n - 1 {
                out.push_str(&tmpline[i].to_string());
                out.push('\n');
            } else {
                out.push_str(&tmpline[i].to_string());
                out.push(' ');
            }
        }
    }

    print!("{out}");
}
