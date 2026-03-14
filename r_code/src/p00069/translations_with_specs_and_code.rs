use std::io::{self, BufRead};

const MAX_ROWS: usize = 11;
const MAX_COLS: usize = 31;

fn check_amida(amida: &[[char; MAX_COLS]; MAX_ROWS], temp: &[usize], atari: usize, m: usize, d: usize) -> bool {
    let mut temp = temp.to_vec(); // Create a mutable copy of temp
    for k in 0..d {
        for l in 0..(n - 1) {
            if amida[k][l] == '1' {
                temp[l] += temp[l + 1];
                temp.swap(l, l + 1); // Swap values in a safe way
            }
        }
    }
    temp[atari - 1] == m
}

fn ins_amida(amida: &mut [[char; MAX_COLS]; MAX_ROWS], temp: &[usize], a: usize, b: usize, atari: usize, m: usize, d: usize) -> bool {
    amida[a][b] = '1';
    let result = check_amida(amida, temp, atari, m, d);
    amida[a][b] = '0'; // Reset the grid
    result
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut amida = [['0'; MAX_COLS]; MAX_ROWS];
    let mut temp = [0; 10];
    let mut keep = [0; 10];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        for i in 0..n {
            temp[i] = i + 1;
            keep[i] = i + 1;
        }

        let m: usize = iter.next().unwrap().parse().unwrap();
        let atari: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();

        for i in 0..d {
            let row: String = reader.lines().next().unwrap().unwrap();
            amida[i] = row.chars().collect::<Vec<char>>().try_into().unwrap();
        }

        let mut flag = false;

        if check_amida(&amida, &temp, atari, m, d) {
            println!("0");
            flag = true;
        }

        if !flag {
            for i in 0..d {
                for j in 0..(n - 1) {
                    temp.copy_from_slice(&keep); // Reset temp to keep
                    if amida[i][j] == '1' {
                        continue;
                    }
                    if j > 0 && amida[i][j - 1] == '1' {
                        continue;
                    }
                    if j < n - 2 && amida[i][j + 1] == '1' {
                        continue;
                    }
                    if ins_amida(&mut amida, &temp, i, j, atari, m, d) {
                        println!("{} {}", i + 1, j + 1);
                        flag = true;
                        break;
                    }
                }
                if flag {
                    break;
                }
            }
        }

        if !flag {
            println!("1");
        }
    }
}
```

### Notes:
1. **Grid Representation**: The grid is represented as a 2D array of characters, with a maximum size defined by constants.
2. **Functionality**: The `check_amida` and `ins_amida` functions are translated to maintain the same logic as in the C code.
3. **Input Handling**: The input is read using `BufRead` for efficiency, and the program handles termination gracefully when `n` is zero.
4. **Safety**: The code avoids unsafe operations and uses safe Rust constructs, such as `Vec` and array slices.
5. **Output**: The output format is preserved exactly as specified, including whitespace and newlines.
