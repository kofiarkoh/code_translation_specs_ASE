use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like C's %d.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: i32 = it.next().unwrap().parse().unwrap();
    let w: i32 = it.next().unwrap().parse().unwrap();
    let k: i32 = it.next().unwrap().parse().unwrap();

    // Preserve C int overflow/wraparound behavior by using wrapping arithmetic.
    for i in 0..=h {
        for j in 0..=w {
            let now = (w.wrapping_mul(i))
                .wrapping_add(h.wrapping_mul(j))
                .wrapping_sub(2i32.wrapping_mul(i).wrapping_mul(j));

            if now == k {
                print!("Yes\n");
                return;
            }
        }
    }

    print!("No\n");
}
