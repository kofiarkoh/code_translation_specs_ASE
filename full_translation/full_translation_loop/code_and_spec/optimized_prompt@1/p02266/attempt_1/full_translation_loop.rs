use std::io::{self, BufRead};

#[derive(Debug)]
struct Stack {
    data: usize,
    next: Option<Box<Stack>>,
}

#[derive(Debug)]
struct Pool {
    area: usize,
    x: usize,
    next: Option<Box<Pool>>,
}

fn output(pool: &Option<Box<Pool>>) {
    if let Some(p) = pool {
        output(&p.next);
        print!(" {}", p.area);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut stack: Option<Box<Stack>> = None;
    let mut pool: Option<Box<Pool>> = None;
    let mut x = 1;

    for byte in reader.bytes() {
        if let Ok(in_char) = byte {
            match in_char as char {
                '\\' => {
                    let new_stack = Box::new(Stack {
                        data: x,
                        next: stack.take(),
                    });
                    stack = Some(new_stack);
                }
                '_' => {}
                '/' => {
                    if let Some(mut s) = stack {
                        if pool.is_none() {
                            pool = Some(Box::new(Pool {
                                area: x - s.data,
                                x: s.data,
                                next: None,
                            }));
                        } else {
                            if s.data < pool.as_ref().unwrap().x {
                                let mut p = pool.as_mut().unwrap().next.take();
                                while let Some(mut pp) = p {
                                    if pp.x > s.data {
                                        pp.area += pool.as_ref().unwrap().area;
                                        p = pp.next.take();
                                    } else {
                                        break;
                                    }
                                }
                                pool.as_mut().unwrap().area += x - s.data;
                                pool.as_mut().unwrap().x = s.data;
                            } else {
                                let new_pool = Box::new(Pool {
                                    area: x - s.data,
                                    x: s.data,
                                    next: pool.take(),
                                });
                                pool = Some(new_pool);
                            }
                        }
                        stack = s.next;
                    }
                }
                _ => {}
            }
            x += 1;
        }
    }

    let mut sum = 0;
    let mut count = 0;
    let mut p = &pool;

    while let Some(ref pp) = p {
        sum += pp.area;
        count += 1;
        p = &pp.next;
    }

    println!("{}\n{}", sum, count);
    output(&pool);
    println!();
}
