use std::io;

fn main() {
    let mut input = String::new();
    // Read a line of input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();

    // Check conditions and print the corresponding output
    if a == 1 && b == 0 && c == 0 {
        println!("Close");
    } else if a == 0 && b == 1 && c == 0 {
        println!("Close");
    } else if a == 1 && b == 1 && c == 0 {
        println!("Open");
    } else if a == 0 && b == 0 && c == 1 {
        println!("Open");
    } else if a == 0 && b == 0 && c == 0 {
        println!("Close");
    }
}
