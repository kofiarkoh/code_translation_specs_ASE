use std::io::{self, Write};

const MAX_D: usize = 11;
const MAX_N: usize = 31;

fn check_amida(amida: &[[char; MAX_N]; MAX_D], temp: &mut [i32; 10], atari: usize, m: i32, d: usize, n: usize) -> i32 {
    for k in 0..d {
        for l in 0..n - 1 {
            if amida[k][l] == '1' {
                temp[l] += temp[l + 1];
                temp[l + 1] = temp[l] - temp[l + 1];
                temp[l] -= temp[l + 1];
            }
        }
    }
    if temp[atari - 1] == m {
        1
    } else {
        0
    }
}

fn ins_amida(amida: &mut [[char; MAX_N]; MAX_D], temp: &mut [i32; 10], a: usize, b: usize, atari: usize, m: i32, d: usize, n: usize) -> i32 {
    amida[a][b] = '1';
    let res = check_amida(amida, temp, atari, m, d, n);
    amida[a][b] = '0';
    res
}

fn main() {
    let mut input = String::new();
    let mut amida = [['0'; MAX_N]; MAX_D];
    let mut temp = [0; 10];
    let mut keep = [0; 10];

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        for i in 0..n {
            temp[i] = (i + 1) as i32;
            keep[i] = (i + 1) as i32;
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let m: i32 = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let atari: usize = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let d: usize = input.trim().parse().unwrap();

        for i in 0..d {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            amida[i] = input.trim().chars().collect::<Vec<char>>().try_into().unwrap();
        }

        let mut flag = 0;

        if check_amida(&amida, &mut temp, atari, m, d, n) == 1 {
            println!("0");
            flag = 1;
        }

        if flag == 0 {
            for i in 0..d {
                for j in 0..n - 1 {
                    temp.copy_from_slice(&keep);
                    if amida[i][j] == '1' {
                        continue;
                    }
                    if j > 0 && amida[i][j - 1] == '1' {
                        continue;
                    }
                    if j != n - 2 && amida[i][j + 1] == '1' {
                        continue;
                    }
                    if ins_amida(&mut amida, &mut temp, i, j, atari, m, d, n) == 1 {
                        println!("{} {}", i + 1, j + 1);
                        flag = 1;
                        break;
                    }
                }
                if flag == 1 {
                    break;
                }
            }
        }

        if flag == 0 {
            println!("1");
        }
    }
}
