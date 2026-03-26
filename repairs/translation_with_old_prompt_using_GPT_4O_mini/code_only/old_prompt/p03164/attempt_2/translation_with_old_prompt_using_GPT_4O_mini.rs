use std::io;

const MEW: usize = 112345;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let w: Vec<i32> = iter.by_ref().take(n).map(|x| x.parse().expect("Invalid input")).collect();
    let v: Vec<i32> = iter.by_ref().take(n).map(|x| x.parse().expect("Invalid input")).collect();

    let W: i32 = 1_000_000; // Placeholder for W, as it was not defined in the original C code
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
