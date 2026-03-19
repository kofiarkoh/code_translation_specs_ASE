use std::io::{self, BufRead};

const P: i64 = 1_000_000_007;

fn mod_value(a: i64) -> i64 {
    let mut a = a % P;
    if a < 0 {
        a += P;
    }
    a
}

fn power(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        power(mod_value(a * a), n / 2)
    } else {
        mod_value(a * power(a, n - 1))
    }
}

fn inverse(a: i64) -> i64 {
    power(a, P - 2)
}

fn max(a: i64, b: i64) -> i64 {
    if a >= b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    let h = nums.next().unwrap();
    let w = nums.next().unwrap();
    let k = nums.next().unwrap();

    let max_hw = max(h, w);
    let mut s: Vec<Vec<char>> = vec![vec![' '; (max_hw + 1) as usize]; max_hw as usize];

    for i in 0..h {
        let line = lines.next().unwrap().unwrap();
        s[i as usize] = line.chars().collect();
    }

    if k == 0 {
        println!("1");
        return;
    }

    let mut f_yoko = 0;
    let mut f_tate = 0;

    for i in 0..h {
        if s[i as usize][0] == '#' && s[i as usize][(w - 1) as usize] == '#' {
            f_yoko = 1;
        }
    }

    for j in 0..w {
        if s[0][j as usize] == '#' && s[(h - 1) as usize][j as usize] == '#' {
            f_tate = 1;
        }
    }

    if f_yoko == 1 && f_tate == 1 {
        println!("1");
        return;
    }

    if f_tate == 1 {
        for i in 0..max_hw {
            for j in 0..max_hw {
                if i < j {
                    s[i as usize].swap(j as usize, i as usize);
                }
            }
        }
        let temp = h;
        h = w;
        w = temp;
    }

    let mut x = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for i in 0..h {
        let mut x_sub = 0;
        for j in 0..w {
            if s[i as usize][j as usize] == '#' {
                x_sub += 1;
            }
        }
        let mut e = 0;
        for j in 1..w {
            if s[i as usize][(j - 1) as usize] == '#' && s[i as usize][j as usize] == '#' {
                e += 1;
            }
        }
        if x_sub == w {
            c += 1;
        } else {
            a += x_sub - e;
            if s[i as usize][0] == '#' && s[i as usize][(w - 1) as usize] == '#' {
                a -= 2;
                b += 1;
            }
        }
        x += x_sub;
    }

    if f_yoko == 0 && f_tate == 0 {
        println!("{}", power(x, k - 1));
    } else if a == 0 && b == 0 {
        println!("{}", power(c, k - 1));
    } else {
        let result = mod_value(mod_value((a + b) * power(x, k - 1) + (x - (a + 2 * b + c)) * power(b + c, k - 1)) * inverse(x - (b + c)));
        println!("{}", result);
    }
}
