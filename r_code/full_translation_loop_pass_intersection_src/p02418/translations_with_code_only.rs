use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse whitespace-separated tokens like scanf("%s").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let str1 = match it.next() {
        Some(s) => s,
        None => return, // If no input, C would have undefined behavior; we just exit.
    };
    let str2 = match it.next() {
        Some(s) => s,
        None => return,
    };

    // C code:
    // strcpy(str, str1); strcat(str, str1);
    // Equivalent to str = str1 + str1.
    let doubled = format!("{}{}", str1, str1);

    // C strstr checks for substring occurrence.
    if doubled.contains(str2) {
        print!("Yes\n");
    } else {
        print!("No\n");
    }
}
