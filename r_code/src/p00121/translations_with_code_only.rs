use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Puzzle {
    num: [[i32; 4]; 2],
    time: i32,
    hash: i32,
    x: i32,
    y: i32,
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            num: [[0; 4]; 2],
            time: 0,
            hash: 0,
            x: 0,
            y: 0,
        }
    }
}

fn generate_hash(num: &[[i32; 4]; 2]) -> i32 {
    // Emulate the C code: build decimal number by concatenating the 8 digits
    let mut ret: i32 = 0;
    for i in 0..8 {
        ret = ret * 10 + num[i / 4][i % 4];
    }
    ret
}

fn main() {
    // Read entire stdin as C's scanf loop reads line-by-line until EOF.
    // We'll split by newline and emulate scanf("%[^\n]") which reads a full line excluding newline.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    // Split preserving empty lines similarly to how scanf would behave:
    // scanf("%[^\n]") does not match an empty line (leading '\n'), but typical problem inputs
    // do not include empty lines. We'll process each line separated by '\n' and skip any final empty after trailing newline.
    let mut lines: Vec<&str> = input.split('\n').collect();
    // If input ends with a newline, split produced a trailing empty string; remove it to emulate scanf behavior
    if let Some(last) = lines.last() {
        if last.is_empty() {
            lines.pop();
        }
    }

    const QUEUE_SIZE: usize = 50000;
    const V_SIZE: usize = 76_543_210; // same size as static char v[76543210];

    // Pre-allocate queue and visited vector
    let mut queue: Vec<Puzzle> = vec![Puzzle::default(); QUEUE_SIZE];
    let mut head: usize = 0;
    let mut tail: usize = 0;

    // visited vector: 0/1 as in C's char array
    let mut v: Vec<u8> = vec![0u8; V_SIZE];

    // Movement deltas as in C
    let dx = [1i32, 0, -1, 0];
    let dy = [0i32, 1, 0, -1];

    // For each input line
    for line in lines {
        // The original C code used scanf("%[^\n]", str) which reads the line excluding the newline.
        // Here `line` is that string (without '\n').
        let s = line;

        // Initialize first puzzle
        let mut first = Puzzle::default();
        // Parse characters at indices j*2 + i*8, subtracting '0'
        for i in 0..2 {
            for j in 0..4 {
                let idx = j * 2 + i * 8;
                // Mirror C behavior: assume input well-formed such that idx is valid.
                let byte = s.as_bytes()[idx];
                first.num[i][j] = (byte - b'0') as i32;
                if first.num[i][j] == 0 {
                    first.x = j as i32;
                    first.y = i as i32;
                }
            }
        }

        // Clear visited vector (memset to zero in C)
        for b in v.iter_mut() {
            *b = 0;
        }

        first.time = 0;
        head = 0;
        tail = 0;
        first.hash = generate_hash(&first.num);

        // enqueue initial (note: C code did NOT mark initial state visited here)
        queue[tail % QUEUE_SIZE] = first;
        tail = tail.wrapping_add(1); // wrapping_add to emulate unbounded increment (modulo used for index)

        // BFS loop
        let mut temp = Puzzle::default();
        loop {
            // dequeue
            temp = queue[head % QUEUE_SIZE];
            head = head.wrapping_add(1);

            // check goal
            if temp.hash == 1_234_567 {
                break;
            }

            for k in 0..4 {
                let mut next = temp;
                next.time = temp.time + 1;
                next.x = temp.x + dx[k];
                next.y = temp.y + dy[k];

                if 0 <= next.x && next.x < 4 && 0 <= next.y && next.y < 2 {
                    // XOR swap between next.num[temp.y][temp.x] and next.num[next.y][next.x]
                    // Emulate the exact C sequence:
                    // first.num[temp.y][temp.x] ^= first.num[first.y][first.x];
                    // first.num[first.y][first.x] ^= first.num[temp.y][temp.x];
                    // first.num[temp.y][temp.x] ^= first.num[first.y][first.x];
                    // Using i32 bitwise XOR which matches C's behavior on 32-bit ints.
                    let ty = temp.y as usize;
                    let tx = temp.x as usize;
                    let ny = next.y as usize;
                    let nx = next.x as usize;

                    next.num[ty][tx] ^= next.num[ny][nx];
                    next.num[ny][nx] ^= next.num[ty][tx];
                    next.num[ty][tx] ^= next.num[ny][nx];

                    next.hash = generate_hash(&next.num);
                    let hidx = next.hash as usize;
                    // In C, v is indexed directly; assume hash is non-negative and within range.
                    if v[hidx] == 0 {
                        v[hidx] = 1;
                        queue[tail % QUEUE_SIZE] = next;
                        tail = tail.wrapping_add(1);
                    }
                }
            }
        }

        // Print the time (same formatting as C: "%d\n")
        println!("{}", temp.time);
    }
}
