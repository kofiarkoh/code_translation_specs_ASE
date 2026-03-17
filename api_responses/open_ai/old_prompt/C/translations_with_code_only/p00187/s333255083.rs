use std::f64;
use std::io::{self, BufRead};

fn l(ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, dx: i32, dy: i32) -> (bool, f64, f64) {
    let acx = cx - ax;
    let acy = cy - ay;
    let bunbo = (bx - ax) * (dy - cy) - (by - ay) * (dx - cx);
    
    if bunbo != 0 {
        let mut r = (dy - cy) * acx - (dx - cx) * acy;
        let mut s = (by - ay) * acx - (bx - ax) * acy;

        if bunbo < 0 {
            r = -r;
            s = -s;
        }

        let bunbo_abs = bunbo.abs();
        if r >= 0 && r <= bunbo_abs && s >= 0 && s <= bunbo_abs {
            let x = ax as f64 + r as f64 / bunbo_abs as f64 * (bx - ax) as f64;
            let y = ay as f64 + r as f64 / bunbo_abs as f64 * (by - ay) as f64;
            return (true, x, y);
        }
    }
    (false, 0.0, 0.0)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut x1 = [0; 3];
    let mut y1 = [0; 3];
    let mut x2 = [0; 3];
    let mut y2 = [0; 3];
    let mut x = [0.0; 3];
    let mut y = [0.0; 3];
    let mut i = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if nums.len() == 4 {
            x1[i] = nums[0];
            y1[i] = nums[1];
            x2[i] = nums[2];
            y2[i] = nums[3];
            i += 1;

            if i == 3 {
                let mut area = 0.0;
                let mut valid = true;

                for j in 0..3 {
                    let (result, x_val, y_val) = l(
                        x1[j], y1[j], x2[j], y2[j],
                        x1[(j + 1) % 3], y1[(j + 1) % 3], x2[(j + 1) % 3], y2[(j + 1) % 3]
                    );
                    if result {
                        x[j] = x_val;
                        y[j] = y_val;
                    } else {
                        valid = false;
                    }
                }

                if valid {
                    for j in 0..3 {
                        area += (x[j] - x[(j + 1) % 3]) * (y[j] + y[(j + 1) % 3]) / 2.0;
                    }
                    area = area.abs();

                    if area < 1e-6 {
                        println!("kyo");
                    } else if area < 100000.0 - 1e-6 {
                        println!("syo-kichi");
                    } else if area < 1000000.0 - 1e-6 {
                        println!("kichi");
                    } else if area < 1900000.0 - 1e-6 {
                        println!("chu-kichi");
                    } else {
                        println!("dai-kichi");
                    }
                } else {
                    println!("kyo");
                }
                i = 0;
            }
        }
    }
}
