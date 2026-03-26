use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut s = String::new();
    let mut t = String::new();

    io::stdin().read_line(&mut s).unwrap();
    io::stdin().read_line(&mut t).unwrap();

    let mut ans = 0;
    let mut i = n as isize - 1;

    while i >= 0 {
        let mut j = n as isize - 1; // Reset j to the last index of T for each outer loop iteration
        while j >= 0 && s.as_bytes()[i as usize] != t.as_bytes()[j as usize] {
            j -= 1;
        }
        if j >= 0 {
            let mut k = i; // Use a new variable k to track the matching characters
            while j >= 0 && k >= 0 && s.as_bytes()[k as usize] == t.as_bytes()[j as usize] {
                k -= 1;
                j -= 1;
            }
            if j < 0 {
                ans = 2 * n - (n - (k + 1) as usize);
                break;
            }
        } else {
            ans = 2 * n - (n - (i + 1) as usize);
            break;
        }
        i -= 1; // Decrement i for the next iteration of the outer loop
    }

    print!("{}", ans);
}
