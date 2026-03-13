// translated_amida.rs
// Translation of the provided C program to Rust.
// Preserves semantics based on the C code (array sizes, copying behavior, I/O formatting).
// Uses wrapping arithmetic to emulate the original arithmetic-swap behavior.

use std::io::{self, Read};

fn check_amida(
    amida: &[[u8; 31]; 11],
    temp: &mut [i32; 10],
    n: usize,
    d: usize,
    atari: usize,
    m: i32,
) -> bool {
    // Perform the same nested simulation as the C check_amida,
    // using wrapping arithmetic to emulate the C arithmetic swap sequence.
    for k in 0..d {
        for l in 0..(n.saturating_sub(1)) {
            if amida[k][l] == b'1' {
                // temp[l] += temp[l+1];
                temp[l] = temp[l].wrapping_add(temp[l + 1]);
                // temp[l+1] = temp[l] - temp[l+1];
                temp[l + 1] = temp[l].wrapping_sub(temp[l + 1]);
                // temp[l] -= temp[l+1];
                temp[l] = temp[l].wrapping_sub(temp[l + 1]);
            }
        }
    }
    // In C: if (temp[atari - 1] == m) return 1; else 0;
    temp[atari - 1] == m
}

fn ins_amida(
    amida: &mut [[u8; 31]; 11],
    a: usize,
    b: usize,
    temp: &mut [i32; 10],
    n: usize,
    d: usize,
    atari: usize,
    m: i32,
) -> bool {
    // Set the position, call check_amida, then reset to '0'.
    amida[a][b] = b'1';
    let res = check_amida(amida, temp, n, d, atari, m);
    amida[a][b] = b'0';
    res
}

fn main() {
    // Read all input tokens separated by whitespace (behaves like scanf in the C code).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    // Arrays sized as in the C code: amida[11][31], temp[10], keep[10].
    let mut amida: [[u8; 31]; 11] = [[b'0'; 31]; 11];
    let mut temp: [i32; 10] = [0; 10];
    let mut keep: [i32; 10] = [0; 10];

    loop {
        let n_token = toks.next();
        if n_token.is_none() {
            break;
        }
        let n: usize = match n_token.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n == 0 {
            break;
        }

        // Initialize only the first n entries, as in the C code.
        for i in 0..n {
            temp[i] = (i as i32) + 1;
            keep[i] = (i as i32) + 1;
        }

        // Read m, atari, d
        let m: i32 = toks.next().unwrap().parse().unwrap();
        let atari_i32: i32 = toks.next().unwrap().parse().unwrap();
        let atari: usize = atari_i32 as usize;
        let d: usize = toks.next().unwrap().parse().unwrap();

        // Read d amida rows. Copy characters into amida[i][..].
        // We set the bytes where provided; other positions remain as initialized.
        for i in 0..d {
            if let Some(s) = toks.next() {
                let bytes = s.as_bytes();
                let len = bytes.len().min(31);
                for j in 0..len {
                    amida[i][j] = bytes[j];
                }
                // If the provided string is shorter than previous contents,
                // we do not explicitly clear the remaining positions to mimic C's behavior closely.
                // (Globals in C start zero; here we initialized to '0', which also != '1'.)
            } else {
                // No more tokens; but to be robust, leave row as initialized.
            }
        }

        let mut flag = 0;

        // Use check_amida with current temp.
        if check_amida(&amida, &mut temp, n, d, atari, m) {
            println!("0");
            flag = 1;
        }

        if flag == 0 {
            'outer: for i in 0..d {
                for j in 0..(n.saturating_sub(1)) {
                    // memcpy(temp, keep, sizeof(keep)) -> copy entire 10 elements
                    temp.copy_from_slice(&keep);

                    if amida[i][j] == b'1' {
                        continue;
                    }
                    if j > 0 {
                        if amida[i][j - 1] == b'1' {
                            continue;
                        }
                    }
                    if j != n.saturating_sub(2) {
                        if amida[i][j + 1] == b'1' {
                            continue;
                        }
                    }
                    if ins_amida(&mut amida, i, j, &mut temp, n, d, atari, m) {
                        // Print i+1 j+1 (1-based)
                        println!("{} {}", i + 1, j + 1);
                        flag = 1;
                        break 'outer;
                    }
                }
            }
        }

        if flag == 0 {
            println!("1");
        }
    }
}
