use std::io::{self, Read, Write};

fn main() {
    let a: &[u8] = b"\n ',-.?ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let b: [&[u8]; 32] = [
        b"101", b"101", b"000000", b"000011", b"10010001", b"010001", b"000001", b"100101",
        b"10011010", b"0101", b"0001", b"110", b"01001", b"10011011", b"010000", b"0111",
        b"10011000", b"0110", b"00100", b"10011001", b"10011110", b"00101", b"111",
        b"10011111", b"1000", b"00110", b"00111", b"10011100", b"10011101", b"000010",
        b"10010010", b"10010011", b"10010000"
    ];
    let c: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ .,-'?";
    let mut d = vec![0u8; 4096];
    let mut i = 0;

    // Read input until newline
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for k in input.bytes() {
        if k == b'\n' {
            break;
        }
        let mut j = 0;
        while a[j] != k {
            j += 1;
        }
        let p = b[j];
        for &byte in p {
            d[i] = byte;
            i += 1;
        }
    }

    // Pad with '0' until i is a multiple of 5
    while i % 5 != 0 {
        d[i] = b'0';
        i += 1;
    }
    d[i] = 0; // Null-terminate the array

    i = 0;
    while d[i] != 0 {
        let mut k = 0;
        let mut j = 4;
        loop {
            k |= (d[i] == b'1') as usize << j;
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        // Output the corresponding character
        print!("{}", c[k] as char);
    }
    println!(); // Print newline at the end
}
