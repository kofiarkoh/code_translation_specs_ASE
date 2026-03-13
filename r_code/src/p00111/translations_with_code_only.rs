use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // encode table: corresponds to C's char encode[64][6]
    // indexed by (byte - b' ') where byte is ASCII of input character
    let encode: [&str; 64] = [
        "11010", "", "", "", "", "", "", "11110", "", "", "", "", "11100", "11101", "11011", "",
        "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "11111", "", "00000",
        "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001",
        "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010",
        "10011", "10100", "10101", "10110", "10111", "11000", "11001", "", "", "", "", "",
    ];

    // decode mapping: emulate C's large decode[] array but only store defined entries.
    // Keys are decimal integers like 21121112 mapping to characters.
    let mut decode: HashMap<usize, char> = HashMap::new();
    decode.insert(212, ' ');
    decode.insert(111111, '\'');
    decode.insert(111122, ',');
    decode.insert(21121112, '-');
    decode.insert(121112, '.');
    decode.insert(111112, '?');
    decode.insert(211212, 'A');
    decode.insert(21122121, 'B');
    decode.insert(1212, 'C');
    decode.insert(1112, 'D');
    decode.insert(221, 'E');
    decode.insert(12112, 'F');
    decode.insert(21122122, 'G');
    decode.insert(121111, 'H');
    decode.insert(1222, 'I');
    decode.insert(21122111, 'J');
    decode.insert(1221, 'K');
    decode.insert(11211, 'L');
    decode.insert(21122112, 'M');
    decode.insert(21122221, 'N');
    decode.insert(11212, 'O');
    decode.insert(222, 'P');
    decode.insert(21122222, 'Q');
    decode.insert(2111, 'R');
    decode.insert(11221, 'S');
    decode.insert(11222, 'T');
    decode.insert(21122211, 'U');
    decode.insert(21122212, 'V');
    decode.insert(111121, 'W');
    decode.insert(21121121, 'X');
    decode.insert(21121122, 'Y');
    decode.insert(21121111, 'Z');

    let stdin = io::stdin();
    let mut out = String::new();
    for line_res in stdin.lock().lines() {
        // In the C code fgets would include the trailing newline in tex.
        // The original loop likely intended to ignore that newline when encoding.
        // To preserve the practical behavior, we use the line without the trailing newline.
        // (This matches common translations and avoids undefined negative indexing.)
        let tex = match line_res {
            Ok(s) => s,
            Err(_) => continue,
        };

        // Build the concatenated code string by mapping each character via encode.
        let mut code = String::new();
        for &b in tex.as_bytes() {
            // compute index = b - ' '
            if b >= b' ' {
                let idx = (b - b' ') as usize;
                if idx < encode.len() {
                    code.push_str(encode[idx]);
                } else {
                    // If out of range, behave as if encode entry is empty (no addition).
                }
            } else {
                // If byte < space, behave as if encode entry is empty (avoid negative index).
            }
        }

        // Now walk the code string, building numeric keys as in C:
        // stk = stk*10 + (code[i]-'0') + 1;
        // if decode[stk] > 0 -> output decode[stk]; stk=0;
        let mut stk: usize = 0;
        for &b in code.as_bytes() {
            // b is '0' or '1' in practice
            let digit = (b - b'0') as usize;
            stk = stk * 10 + digit + 1; // adds 1 => digits become 1 or 2
            if let Some(&ch) = decode.get(&stk) {
                out.push(ch);
                stk = 0;
            }
        }

        // The original C code attempted to print a newline after each processed input line.
        // It had a faulty printf("%\n"); but the intended/observed effect is a newline.
        out.push('\n');
    }

    // Write accumulated output to stdout
    print!("{}", out);
}
