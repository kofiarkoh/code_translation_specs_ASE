use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as C-like whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C code uses implicit-int globals: n, x; and main(y) where y is an int parameter.
    // We model all as i32 to match typical C `int` behavior and preserve wrapping.
    let n0: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut n = n0;
    let mut x: i32 = 0;

    // for (scanf("%d",&n); n--; printf("%d\n",(x-1)%y+1)) scanf("%d%d",&x,&y);
    // Note: In C, the loop condition is `n--` (post-decrement): it tests the old value.
    // So it runs exactly n0 times if n0 > 0; if n0 <= 0, it runs 0 times.
    while n != 0 {
        n = n.wrapping_sub(1);

        // Body: scanf("%d%d",&x,&y);
        let x_in: i32 = it.next().unwrap().parse().unwrap();
        let y: i32 = it.next().unwrap().parse().unwrap();
        x = x_in;

        // Iteration expression: printf("%d\n",(x-1)%y+1)
        // Use wrapping arithmetic to emulate C signed overflow behavior as closely as possible.
        // Note: C `%` with negative operands truncates toward zero; Rust matches that.
        let val = x.wrapping_sub(1).wrapping_rem(y).wrapping_add(1);
        print!("{val}\n");
    }

    // C calls exit(0); returning from main is equivalent.
}
