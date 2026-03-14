use std::io::{self, BufRead};

#[derive(Copy, Clone)]
struct XY {
    x: f64,
    y: f64,
}

fn qsort_comp(a: &XY, b: &XY) -> std::cmp::Ordering {
    if a.x > b.x {
        std::cmp::Ordering::Greater
    } else if a.x < b.x {
        std::cmp::Ordering::Less
    } else {
        if a.y > b.y {
            std::cmp::Ordering::Greater
        } else if a.y < b.y {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut zahyou: Vec<XY> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let zahyou_num: i32 = line.trim().parse().unwrap();
        if zahyou_num == 0 {
            break;
        }

        zahyou.clear();
        for _ in 0..zahyou_num {
            let line = reader.lines().next().unwrap().unwrap();
            let coords: Vec<&str> = line.split(',').collect();
            let x: f64 = coords[0].parse().unwrap();
            let y: f64 = coords[1].parse().unwrap();
            zahyou.push(XY { x, y });
        }

        zahyou.sort_by(|a, b| qsort_comp(a, b));

        // Lower hull
        let mut th_down: Vec<XY> = Vec::new();
        th_down.push(zahyou[0]);
        for i in 1..zahyou_num as usize {
            while th_down.len() > 1 {
                let k1 = if zahyou[i].x == th_down[th_down.len() - 1].x {
                    if zahyou[i].y < th_down[th_down.len() - 1].y {
                        -1e+200
                    } else if zahyou[i].y > th_down[th_down.len() - 1].y {
                        1e+200
                    } else {
                        0.0
                    }
                } else {
                    (zahyou[i].y - th_down[th_down.len() - 1].y) /
                    (zahyou[i].x - th_down[th_down.len() - 1].x)
                };

                let k2 = if th_down[th_down.len() - 1].x == th_down[th_down.len() - 2].x {
                    if th_down[th_down.len() - 1].y < th_down[th_down.len() - 2].y {
                        -1e+200
                    } else if th_down[th_down.len() - 1].y > th_down[th_down.len() - 2].y {
                        1e+200
                    } else {
                        0.0
                    }
                } else {
                    (th_down[th_down.len() - 1].y - th_down[th_down.len() - 2].y) /
                    (th_down[th_down.len() - 1].x - th_down[th_down.len() - 2].x)
                };

                if k1 >= k2 {
                    break;
                }
                th_down.pop();
            }
            th_down.push(zahyou[i]);
        }

        // Upper hull
        let mut th_up: Vec<XY> = Vec::new();
        th_up.push(zahyou[zahyou_num as usize - 1]);
        for i in (0..zahyou_num as usize - 1).rev() {
            while th_up.len() > 1 {
                let k1 = if zahyou[i].x == th_up[th_up.len() - 1].x {
                    if zahyou[i].y < th_up[th_up.len() - 1].y {
                        1e+200
                    } else if zahyou[i].y > th_up[th_up.len() - 1].y {
                        -1e+200
                    } else {
                        0.0
                    }
                } else {
                    (zahyou[i].y - th_up[th_up.len() - 1].y) /
                    (zahyou[i].x - th_up[th_up.len() - 1].x)
                };

                let k2 = if th_up[th_up.len() - 1].x == th_up[th_up.len() - 2].x {
                    if th_up[th_up.len() - 1].y < th_up[th_up.len() - 2].y {
                        1e+200
                    } else if th_up[th_up.len() - 1].y > th_up[th_up.len() - 2].y {
                        -1e+200
                    } else {
                        0.0
                    }
                } else {
                    (th_up[th_up.len() - 1].y - th_up[th_up.len() - 2].y) /
                    (th_up[th_up.len() - 1].x - th_up[th_up.len() - 2].x)
                };

                if k1 >= k2 {
                    break;
                }
                th_up.pop();
            }
            th_up.push(zahyou[i]);
        }

        println!("{}", zahyou_num - (th_down.len() + th_up.len() - 2));
    }
}
