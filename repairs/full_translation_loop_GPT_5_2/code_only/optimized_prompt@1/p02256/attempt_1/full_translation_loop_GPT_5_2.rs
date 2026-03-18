use std::io::{self, Read};

type U32 = u32;
type U08 = u8;

const INPUT_NUM_MIN: U32 = 1;
const INPUT_NUM_MAX: U32 = 1_000_000_000;

fn calc_gcd_u32(mut large: U32, mut small: U32) -> U32 {
    let mut rem: U32 = 0;

    loop {
        if small == 0 {
            break;
        } else {
            rem = large % small;
            if rem == 0 {
                break;
            } else {
                large = small;
                small = rem;
            }
        }
    }

    small
}

fn main() {
    // Read entire stdin; if parsing fails or insufficient tokens, behave like C code
    // where u32_x/u32_y remain 0 (since scanf return value is ignored).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    // C uses scanf("%d") into unsigned int; negative inputs are implementation-defined.
    // We approximate by parsing as i64 then casting to u32 (two's-complement wrap),
    // and if parsing fails, keep default 0.
    let parse_u32_like_c = |s: Option<&str>| -> U32 {
        if let Some(tok) = s {
            if let Ok(v) = tok.parse::<i64>() {
                v as u32
            } else {
                0
            }
        } else {
            0
        }
    };

    let u32_x: U32 = parse_u32_like_c(it.next());
    let u32_y: U32 = parse_u32_like_c(it.next());

    let mut return_val: U08 = 0;

    if (u32_x < INPUT_NUM_MIN)
        || (INPUT_NUM_MAX < u32_x)
        || (u32_y < INPUT_NUM_MIN)
        || (INPUT_NUM_MAX < u32_y)
    {
        return_val = 1;
    } else {
        let gcd = if u32_x > u32_y {
            calc_gcd_u32(u32_x, u32_y)
        } else {
            calc_gcd_u32(u32_y, u32_x)
        };
        print!("{}\n", gcd);
    }

    std::process::exit(return_val as i32);
}
