use std::io;

const MEW: usize = 112345;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Read N and W from input
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let W: i32 = iter.next().unwrap().parse().expect("Invalid input"); // Read W from input

    // Read weights and values
    let mut w = vec![0; n];
    let mut v = vec![0; n];
    for i in 0..n {
        w[i] = iter.next().unwrap().parse().expect("Invalid input");
        v[i] = iter.next().unwrap().parse().expect("Invalid input");
    }

    let mut ayaneru = vec![W + 1; MEW];
    ayaneru[0] = 0;

    for i in 0..n {
        for j in (v[i]..=MEW as i32 - 1).rev() {
            if ayaneru[(j - v[i]) as usize] + w[i] < ayaneru[j as usize] {
                ayaneru[j as usize] = ayaneru[(j - v[i]) as usize] + w[i];
            }
        }
    }

    for i in (0..MEW).rev() {
        if ayaneru[i] <= W {
            println!("{}", i);
            return;
        }
    }
    println!("error!");
}
