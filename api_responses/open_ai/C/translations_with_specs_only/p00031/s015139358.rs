use std::io::{self, BufRead};

fn main() {
    // Predefined weights corresponding to the binary representation
    const WEIGHTS: [usize; 64] = [
        1, 2, 4, 8, 16, 32, 64, 128,
        256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
        65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608,
        16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824, 2147483648,
        4294967296, 8589934592, 17179869184, 34359738368, 68719476736, 137438953472, 274877906944, 549755813888,
        1099511627776, 2199023255552, 4398046511104, 8796093022208, 17592186044416, 35184372088832, 70368744177664, 140737488355328,
        281474976710656, 562949953421312, 1125899906842624, 2251799813685248, 4503599627370496, 9007199254740992, 18014398509481984, 36028797018963968,
        72057594037927936, 144115188075855872, 288230376151711744, 576460752303423488, 1152921504606846976, 2305843009213693952, 4611686018427387904, 9223372036854775808,
    ];

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                if let Ok(num) = input.trim().parse::<u64>() {
                    let mut weights: Vec<usize> = Vec::new();
                    let mut index = 0;

                    // Process each bit of the number
                    while index < WEIGHTS.len() && num > 0 {
                        if num & 1 == 1 {
                            weights.push(WEIGHTS[index]);
                        }
                        num >>= 1; // Shift right to check the next bit
                        index += 1;
                    }

                    // Output the weights corresponding to the set bits
                    if !weights.is_empty() {
                        println!("{}", weights.iter().map(usize::to_string).collect::<Vec<String>>().join(" "));
                    }
                }
            }
            Err(_) => {
                // Undefined behavior on invalid input, continue processing
                continue;
            }
        }
    }
}
