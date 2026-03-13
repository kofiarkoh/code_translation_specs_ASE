use std::io::{self, Read, Write};

fn main() {
    // Original C program treats all data as raw bytes; replicate that with byte slices.
    // Mapping arrays taken verbatim from the C source.
    let a: &[u8] = b"\n ',-.?ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let b: [&[u8]; 32] = [
        b"101", b"101", b"000000", b"000011", b"10010001", b"010001", b"000001", b"100101",
        b"10011010", b"0101", b"0001", b"110", b"01001", b"10011011", b"010000", b"0111",
        b"10011000", b"0110", b"00100", b"10011001", b"10011110", b"00101", b"111",
        b"10011111", b"1000", b"00110", b"00111", b"10011100", b"10011101", b"000010",
        b"10010010", b"10010011", b"10010000",
    ];
    let c: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ .,-'?";

    // Read bytes from stdin up to and including the first newline (if any).
    // The original C code reads with getc and stops when it encounters '\n'.
    // Using read_until ensures we stop at newline or EOF; this is the practical behavior
    // consistent with typical usage of the original program.
    let mut stdin = io::stdin();
    let mut input: Vec<u8> = Vec::new();
    stdin.read_to_end(&mut input).expect("read failed");

    // Find first newline and consider only bytes before it, matching the C loop:
    // while ((k = getc(stdin)) != '\n') { ... }
    if let Some(pos) = input.iter().position(|&b| b == b'\n') {
        input.truncate(pos);
    } else {
        // No newline: treat all read bytes as input (practical behavior).
        // The original C program's behavior on EOF-without-newline is undefined,
        // so this is a reasonable, safe interpretation.
    }

    // d will hold the concatenated '0'/'1' bits as bytes.
    let mut d: Vec<u8> = Vec::with_capacity(4096);

    for &k in &input {
        // Find index j such that a[j] == k.
        // The C program assumes the character exists in `a`. If it doesn't,
        // position() returns None and unwrap() will panic (similar to the C program's undefined behavior).
        let j = a.iter().position(|&x| x == k).expect("input contains character not in mapping");
        let p = b[j];
        d.extend_from_slice(p);
    }

    // Pad with '0' bytes until length is a multiple of 5.
    while d.len() % 5 != 0 {
        d.push(b'0');
    }

    // Decode each group of 5 bits into an index into `c` and write the resulting byte.
    let mut out: Vec<u8> = Vec::with_capacity(d.len() / 5 + 1);
    let mut i = 0usize;
    while i < d.len() {
        let mut k: u8 = 0;
        // Most-significant bit first: bit positions 4 down to 0.
        for j in 0..5 {
            if d[i + j] == b'1' {
                k |= 1 << (4 - j);
            }
        }
        i += 5;
        // Index into c; k should be in 0..32 as per the C program.
        out.push(c[k as usize]);
    }

    // Write output and a final newline, matching the C program's putc('\n', stdout).
    let mut stdout = io::stdout();
    stdout.write_all(&out).expect("write failed");
    stdout.write_all(b"\n").expect("write failed");
}
