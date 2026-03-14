use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let mut a = [0; 10];
        let mut nflag = 0;

        // Read 9 integers with a space after each
        for j in 0..9 {
            let line = lines.next().unwrap().unwrap();
            a[j] = line.trim().parse().unwrap();
        }
        // Read the 10th integer
        let line = lines.next().unwrap().unwrap();
        a[9] = line.trim().parse().unwrap();

        let mut b = a[0];
        let mut c = 0;

        for j in 1..10 {
            if b < a[j] {
                b = a[j];
            } else if c < a[j] {
                c = a[j];
            } else {
                println!("NO");
                nflag = 1;
                break;
            }
        }
        if nflag != 0 {
            continue;
        }
        println!("YES");
    }
}
