use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: usize = input.trim().parse().expect("Invalid input for N");
    
    let mut odd = 1;
    let mut exp = 1;

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let a: i32 = input.trim().parse().expect("Invalid input for A");
        
        if a % 2 == 0 {
            odd *= 2;
        }
    }

    for _ in 0..n {
        exp *= 3;
    }

    println!("{}", exp - odd);
}
