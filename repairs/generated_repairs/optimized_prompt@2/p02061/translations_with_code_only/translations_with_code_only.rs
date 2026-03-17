use std::io::{self, BufRead};

const MAX_SIZE: usize = 114514;

static mut COUNT: [i32; MAX_SIZE] = [0; MAX_SIZE];
static mut FUKUENN_MISATO: [[i32; 2]; 64] = [[0; 2]; 64];

static mut MIZUKINANA: usize = 0;
static mut UTIYAMAYUMI: i64 = 0;
static mut YOSINAGASAYURI: i64 = 0;

fn minaseinori(idx: usize, mut acc: i64) {
    unsafe {
        if idx >= MIZUKINANA {
            if acc != UTIYAMAYUMI {
                YOSINAGASAYURI *= acc;
                if YOSINAGASAYURI > UTIYAMAYUMI * 2 {
                    YOSINAGASAYURI = UTIYAMAYUMI * 2;
                }
            }
        } else {
            for i in 0..=FUKUENN_MISATO[idx][1] {
                minaseinori(idx + 1, acc);
                acc *= FUKUENN_MISATO[idx][0] as i64; // Cast to i64 for multiplication
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let q: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut n = vec![0; q as usize];
    let mut max = 0;

    for i in 0..q {
        n[i as usize] = lines.next().unwrap().unwrap().parse().unwrap();
        if max < n[i as usize] {
            max = n[i as usize];
        }
    }

    for i in 2..=max {
        let mut sumipe = i;
        unsafe {
            MIZUKINANA = 0;
            for tomatu in 2..=((sumipe as f64).sqrt() as i32) {
                if sumipe % tomatu == 0 {
                    let mut c = 0;
                    while sumipe % tomatu == 0 {
                        sumipe /= tomatu;
                        c += 1;
                    }
                    FUKUENN_MISATO[MIZUKINANA][0] = tomatu;
                    FUKUENN_MISATO[MIZUKINANA][1] = c;
                    MIZUKINANA += 1;
                }
            }
            if sumipe > 1 {
                FUKUENN_MISATO[MIZUKINANA][0] = sumipe;
                FUKUENN_MISATO[MIZUKINANA][1] = 1;
                MIZUKINANA += 1;
            }
            UTIYAMAYUMI = i as i64;
            YOSINAGASAYURI = 1;
            minaseinori(0, 1);
            COUNT[i as usize] = COUNT[(i - 1) as usize] + (YOSINAGASAYURI >= i as i64 * 2) as i32;
        }
    }

    for i in 0..q {
        println!("{}", unsafe { COUNT[n[i as usize] as usize] });
    }
}
