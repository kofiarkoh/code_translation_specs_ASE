use std::collections::VecDeque;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Puzzle {
    num: [[i32; 4]; 2],
    time: i32,
    hash: i32,
    x: usize,
    y: usize,
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
    let mut queue: VecDeque<Puzzle> = VecDeque::new();
    let mut visited = vec![0; 76543210]; // Using a large enough vector for visited states

    for line in reader.lines() {
        let str = line.unwrap();
        let mut first = Puzzle {
            num: [[0; 4]; 2],
            time: 0,
            hash: 0,
            x: 0,
            y: 0,
        };

        for i in 0..2 {
            for j in 0..4 {
                first.num[i][j] = str.chars().nth(j * 2 + i * 8).unwrap().to_digit(10).unwrap() as i32;
                if first.num[i][j] == 0 {
                    first.x = j;
                    first.y = i;
                }
            }
        }

        visited.fill(0); // Reset visited states
        first.hash = generate_hash(&first.num);
        queue.push_back(first);

        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];

        while let Some(temp) = queue.pop_front() {
            if temp.hash == 1234567 {
                println!("{}", temp.time);
                break;
            }

            for i in 0..4 {
                let mut next = temp.clone();
                next.time += 1;
                next.x = (temp.x as isize + dx[i]) as usize;
                next.y = (temp.y as isize + dy[i]) as usize;

                if next.x < 4 && next.y < 2 {
                    // Swap the zero with the adjacent number
                    next.num[temp.y][temp.x] ^= next.num[next.y][next.x];
                    next.num[next.y][next.x] ^= next.num[temp.y][temp.x];
                    next.num[temp.y][temp.x] ^= next.num[next.y][next.x];

                    next.hash = generate_hash(&next.num);
                    if visited[next.hash as usize] == 0 {
                        visited[next.hash as usize] = 1;
                        queue.push_back(next);
                    }
                }
            }
        }
    }
}
