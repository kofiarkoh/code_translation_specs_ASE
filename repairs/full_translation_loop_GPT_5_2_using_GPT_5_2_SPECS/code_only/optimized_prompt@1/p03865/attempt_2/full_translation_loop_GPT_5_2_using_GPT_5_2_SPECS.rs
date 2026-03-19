use std::io::{self, Read};

fn main() {
    // Emulate: char buf[114514]; n = read(0, buf, 114514);
    let mut buf = vec![0u8; 114_514];
    let n_read = io::stdin().read(&mut buf).unwrap_or(0);

    // C: n--;
    // In the intended problem setting, input is a line ending with '\n',
    // so this effectively drops the newline and makes n the string length.
    let n = (n_read as i64) - 1;

    // C: (buf[0]==buf[n-1]) where n is after decrement.
    // For normal inputs (>=2 bytes including '\n'), this indexes the last character.
    // Keep behavior defined for all inputs without unsafe.
    let b0 = *buf.get(0).unwrap_or(&0);
    let last_idx = n - 1;
    let blast = if last_idx >= 0 {
        *buf.get(last_idx as usize).unwrap_or(&0)
    } else {
        0
    };

    // C precedence: n + ((buf[0]==buf[n-1]) & 1)
    // Since (buf[0]==buf[n-1]) is 0/1, &1 is redundant but preserved.
    let eq01 = if b0 == blast { 1i64 } else { 0i64 };
    let cond = (n + (eq01 & 1)) & 1;

    if cond != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
