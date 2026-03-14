use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut input = String::new();
    
    while reader.read_line(&mut input).unwrap() > 0 {
        let mut parts = input.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        let c: i32 = parts.next().unwrap().parse().unwrap();
        
        let mut t = 0;
        let mut d = 0;

        if a <= 1868 && b <= 9 && c < 8 {
            println!("pre-meiji");
            d = 1;
            t = 1;
        } else if a <= 1912 {
            if a != 1912 {
                print!("meiji ");
                let a = a - 1868 + 1;
                d = 0;
                t = 1;
                println!("{}", a);
            } else if a == 1912 {
                if b < 7 {
                    print!("meiji ");
                    let a = a - 1868 + 1;
                    d = 0;
                    t = 1;
                    println!("{}", a);
                } else if b >= 7 {
                    if c < 30 {
                        print!("meiji ");
                        let a = a - 1868 + 1;
                        d = 0;
                        t = 1;
                        println!("{}", a);
                    }
                }
            }
        }
        
        if a <= 1926 && t == 0 {
            if a != 1926 {
                print!("taisyo ");
                let a = a - 1912 + 1;
                d = 0;
                t = 1;
                println!("{}", a);
            } else if a == 1926 {
                if b < 12 {
                    print!("taisyo ");
                    let a = a - 1912 + 1;
                    d = 0;
                    t = 1;
                    println!("{}", a);
                } else if b == 12 {
                    if c < 25 {
                        print!("taisyo ");
                        let a = a - 1912 + 1;
                        d = 0;
                        t = 1;
                        println!("{}", a);
                    } else if c >= 25 {
                        print!("showa ");
                        let a = a - 1926 + 1;
                        d = 0;
                        t = 1;
                        println!("{}", a);
                    }
                }
            }
        }
        
        if a <= 1989 && t == 0 {
            if a != 1989 {
                print!("showa ");
                let a = a - 1926 + 1;
                d = 0;
                t = 1;
                println!("{}", a);
            } else if a == 1989 {
                if b == 1 {
                    if c < 8 {
                        print!("showa ");
                        let a = a - 1926 + 1;
                        d = 0;
                        t = 1;
                        println!("{}", a);
                    }
                }
            }
        }
        
        if a >= 1989 && t == 0 {
            if a != 1989 {
                print!("heisei ");
                let a = a - 1989 + 1;
                d = 0;
            } else if a == 1989 {
                if b != 1 {
                    print!("heisei ");
                    let a = a - 1989 + 1;
                    d = 0;
                } else if b == 1 {
                    if c >= 8 {
                        print!("heisei ");
                        let a = a - 1989 + 1;
                        d = 0;
                    }
                }
            }
        }
        
        if d == 0 {
            println!("{} {} {}", a, b, c);
        }
        
        input.clear(); // Clear input for the next iteration
    }
}
