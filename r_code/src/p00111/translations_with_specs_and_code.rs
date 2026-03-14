use std::io::{self, BufRead};

const ENCODE: [&str; 64] = [
    "11010", "", "", "", "", "", "", "11110", "", "", "", "", "11100", "11101", "11011", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "11111", "", "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001", "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010", "10011", "10100", "10101", "10110", "10111", "11000", "11001", "", "", "", "", "",
];

const DECODE: [Option<char>; 100_000_000] = {
    let mut arr = [None; 100_000_000];
    arr[212] = Some(' ');
    arr[111111] = Some('\'');
    arr[111122] = Some(',');
    arr[21121112] = Some('-');
    arr[121112] = Some('.');
    arr[111112] = Some('?');
    arr[211212] = Some('A');
    arr[21122121] = Some('B');
    arr[1212] = Some('C');
    arr[1112] = Some('D');
    arr[221] = Some('E');
    arr[12112] = Some('F');
    arr[21122122] = Some('G');
    arr[121111] = Some('H');
    arr[1222] = Some('I');
    arr[21122111] = Some('J');
    arr[1221] = Some('K');
    arr[11211] = Some('L');
    arr[21122112] = Some('M');
    arr[21122221] = Some('N');
    arr[11212] = Some('O');
    arr[222] = Some('P');
    arr[21122222] = Some('Q');
    arr[2111] = Some('R');
    arr[11221] = Some('S');
    arr[11222] = Some('T');
    arr[21122211] = Some('U');
    arr[21122212] = Some('V');
    arr[111121] = Some('W');
    arr[21121121] = Some('X');
    arr[21121122] = Some('Y');
    arr[21121111] = Some('Z');
    arr
};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let tex = line.expect("Failed to read line");
        let mut code = String::new();

        for ch in tex.chars() {
            if let Some(encoded) = ENCODE[ch as usize - ' ' as usize].to_string().as_str() {
                code.push_str(encoded);
            }
        }

        let mut stk = 0;
        for ch in code.chars() {
            stk = stk * 10 + (ch.to_digit(10).unwrap() as usize) + 1;
            if let Some(decoded_char) = DECODE.get(stk) {
                if let Some(c) = decoded_char {
                    print!("{}", c);
                    stk = 0;
                }
            }
        }
        println!("%");
    }
}
