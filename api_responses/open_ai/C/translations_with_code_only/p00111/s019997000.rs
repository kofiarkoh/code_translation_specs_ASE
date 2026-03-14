use std::io::{self, BufRead};

const ENCODE: [&str; 64] = [
    "11010", "", "", "", "", "", "", "11110", "", "", "", "", "11100", "11101", "11011", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "11111", "", "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001", "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010", "10011", "10100", "10101", "10110", "10111", "11000", "11001", "", "", "", "", "",
];

const DECODE: [char; 100_000_000] = {
    let mut arr = ['\0'; 100_000_000];
    arr[212] = ' ';
    arr[111_111] = '\'';
    arr[111_122] = ',';
    arr[211_211_12] = '-';
    arr[121_112] = '.';
    arr[111_112] = '?';
    arr[211_212] = 'A';
    arr[211_221_21] = 'B';
    arr[1212] = 'C';
    arr[1112] = 'D';
    arr[221] = 'E';
    arr[121_12] = 'F';
    arr[211_221_22] = 'G';
    arr[121_111] = 'H';
    arr[1222] = 'I';
    arr[211_221_11] = 'J';
    arr[1221] = 'K';
    arr[11211] = 'L';
    arr[211_221_12] = 'M';
    arr[211_222_21] = 'N';
    arr[11212] = 'O';
    arr[222] = 'P';
    arr[211_222_22] = 'Q';
    arr[2111] = 'R';
    arr[11221] = 'S';
    arr[11222] = 'T';
    arr[211_222_11] = 'U';
    arr[211_222_12] = 'V';
    arr[111_121] = 'W';
    arr[211_211_21] = 'X';
    arr[211_211_22] = 'Y';
    arr[211_211_11] = 'Z';
    arr
};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let tex = line.unwrap();
        let mut code = String::new();

        for c in tex.chars() {
            let index = c as usize - ' ' as usize;
            if index < ENCODE.len() {
                code.push_str(ENCODE[index]);
            }
        }

        let mut stk = 0;
        for c in code.chars() {
            stk = stk * 10 + (c as usize - '0' as usize) + 1;
            if stk < DECODE.len() && DECODE[stk] != '\0' {
                print!("{}", DECODE[stk]);
                stk = 0;
            }
        }
        println!("%");
    }
}
