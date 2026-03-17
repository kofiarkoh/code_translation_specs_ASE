use std::io::{self, Write};

type U32 = u32;
type U08 = u8;

const INPUT_NUM_MIN: U32 = 1; // 入力値の範囲下限
const INPUT_NUM_MAX: U32 = 1_000_000_000; // 入力値の範囲上限

fn calc_gcd_u32(u32_large_num: U32, u32_small_num: U32) -> U32 {
    let mut u32_large_num = u32_large_num;
    let mut u32_small_num = u32_small_num;

    loop {
        if u32_small_num == 0 {
            break;
        } else {
            let u32_rem = u32_large_num % u32_small_num;

            if u32_rem == 0 {
                break;
            } else {
                u32_large_num = u32_small_num;
                u32_small_num = u32_rem;
            }
        }
    }

    u32_large_num
}

fn main() -> Result<(), U08> {
    let mut input = String::new();
    let mut u32_x: U32 = 0;
    let mut u32_y: U32 = 0;

    // 数値入力
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<&str> = input.trim().split_whitespace().collect();

    if nums.len() != 2 {
        return Err(1); // 異常終了
    }

    u32_x = nums[0].parse().unwrap_or(0);
    u32_y = nums[1].parse().unwrap_or(0);

    if (u32_x < INPUT_NUM_MIN || u32_x > INPUT_NUM_MAX) || 
       (u32_y < INPUT_NUM_MIN || u32_y > INPUT_NUM_MAX) {
        return Err(1); // 戻り値に1を設定
    } else {
        let u32_gcd = if u32_x > u32_y {
            calc_gcd_u32(u32_x, u32_y) // 最大公約数の計算
        } else {
            calc_gcd_u32(u32_y, u32_x) // 最大公約数の計算
        };

        println!("{}", u32_gcd); // 最大公約数の表示
    }

    Ok(()) // 正常終了
}
