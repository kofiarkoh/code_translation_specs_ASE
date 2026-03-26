use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut nums = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse number"));

    let a = nums.next().unwrap();
    let b = nums.next().unwrap();
    let c = nums.next().unwrap();

    let (mut sum, mut l) = (0, 0);

    if a > b {
        for i in (1..=b).rev() {
            if a % i == 0 && b % i == 0 {
                sum += 1;
            }

            if sum == c {
                l = i;
                break;
            }
        }
        println!("{}", l);
    } else if a < b {
        for i in (1..=a).rev() {
            if a % i == 0 && b % i == 0 {
                sum += 1;
            }

            if sum == c {
                l = i;
                break;
            }
        }
        println!("{}", l);
    } else { // a == b
        for i in (1..=a).rev() {
            if a % i == 0 && b % i == 0 {
                sum += 1;
            }

            if sum == c {
                l = i;
                break;
            }
        }
        println!("{}", l);
    }
}
