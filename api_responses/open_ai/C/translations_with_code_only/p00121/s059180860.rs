use std::io::{self, BufRead};

#[derive(Clone)]
struct Puzzle {
    num: [[i32; 4]; 2],
    time: i32,
    hash: i32,
    x: usize,
    y: usize,
}

const QUEUE_SIZE: usize = 50000;
static mut QUEUE: [Puzzle; QUEUE_SIZE] = [Puzzle {
    num: [[0; 4]; 2],
    time: 0,
    hash: 0,
    x: 0,
    y: 0,
}; QUEUE_SIZE];
static mut HEAD: usize = 0;
static mut TAIL: usize = 0;

fn enq(t: Puzzle) {
    unsafe {
        QUEUE[TAIL % QUEUE_SIZE] = t;
        TAIL += 1;
    }
}

fn deq() -> Puzzle {
    unsafe {
        let t = QUEUE[HEAD % QUEUE_SIZE].clone();
        HEAD += 1;
        t
    }
}

fn generate_hash(num: &[[i32; 4]; 2]) -> i32 {
    let mut ret = 0;
    for i in 0..8 {
        ret = ret * 10 + num[i / 4][i % 4];
    }
    ret
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut str_buf = String::new();
    let mut v = vec![0; 76543210];

    while reader.read_line(&mut str_buf).unwrap() > 0 {
        let str_data = str_buf.trim();
        let mut first = Puzzle {
            num: [[0; 4]; 2],
            time: 0,
            hash: 0,
            x: 0,
            y: 0,
        };

        for i in 0..2 {
            for j in 0..4 {
                first.num[i][j] = str_data.chars().nth(j * 2 + i * 8).unwrap() as i32 - '0' as i32;
                if first.num[i][j] == 0 {
                    first.x = j;
                    first.y = i;
                }
            }
        }

        v.fill(0);
        first.time = 0;
        unsafe {
            HEAD = 0;
            TAIL = 0;
        }
        first.hash = generate_hash(&first.num);
        enq(first.clone());

        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];

        loop {
            let temp = deq();

            if temp.hash == 1234567 {
                println!("{}", temp.time);
                break;
            }

            for i in 0..4 {
                let mut next = temp.clone();
                next.time = temp.time + 1;
                next.x = (temp.x as isize + dx[i]) as usize;
                next.y = (temp.y as isize + dy[i]) as usize;

                if next.x < 4 && next.y < 2 {
                    // Swap the numbers
                    next.num[temp.y][temp.x] ^= next.num[next.y][next.x];
                    next.num[next.y][next.x] ^= next.num[temp.y][temp.x];
                    next.num[temp.y][temp.x] ^= next.num[next.y][next.x];
                    next.hash = generate_hash(&next.num);

                    if v[next.hash as usize] == 0 {
                        v[next.hash as usize] = 1;
                        enq(next);
                    }
                }
            }
        }
        str_buf.clear(); // Clear the buffer for the next input
    }
}
