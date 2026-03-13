use std::io::{self, Read, Write};

fn main() {
    // Read all input tokens (behaves like repeated scanf("%d", &n))
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let mut out = io::stdout();
    let mut first = true;

    // Process each integer token until a zero is encountered.
    // This matches the C loop `while (scanf("%d", &n) && n) { ... }`
    // which continues only when reading succeeded and n != 0.
    while let Some(tok) = iter.next() {
        // parse token as i32. If parsing fails, stop (mirrors scanf failing).
        let mut n: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n == 0 {
            break;
        }

        // abacus: 8 rows, 6 columns. We keep column 5 as the C code's trailing '\0'
        // by leaving it as 0. We store bytes (u8) for easy writing.
        let mut abacus = [[0u8; 6]; 8];

        // Initialize columns 0..4 of every row to '*'
        for i in 0..8 {
            for j in 0..5 {
                abacus[i][j] = b'*';
            }
        }

        // Row 2, columns 0..4 set to '='
        for j in 0..5 {
            abacus[2][j] = b'=';
        }

        // Process up to 5 digits (least significant first), same logic as C code.
        for i in 0..5 {
            let mut a = n % 10;
            n /= 10;
            let col = 4 - i; // target column
            if a < 5 {
                abacus[1][col as usize] = b' ';
            } else {
                abacus[0][col as usize] = b' ';
                a -= 5;
            }
            // put(3 + a, col, ' ')
            let row = 3 + a;
            // The original C code assumes inputs produce valid indices 3..7.
            // We perform an unsigned index conversion here.
            abacus[row as usize][col as usize] = b' ';
        }

        // Print a blank line between cases, like the C code's `if (f) puts("");`
        if !first {
            out.write_all(b"\n").unwrap();
        }

        // Print each row. The C program uses puts(abacus[i]) and relies on the
        // sixth byte being '\0' to terminate the string, so we write bytes 0..5
        // (the visible 5 chars) and then a newline.
        for i in 0..8 {
            out.write_all(&abacus[i][0..5]).unwrap();
            out.write_all(b"\n").unwrap();
        }

        first = false;
    }

    // flush output
    out.flush().unwrap();
}
