use std::io::{self, BufRead};

fn is_point_in_triangle(px: f64, py: f64, ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> bool {
    let area_abc = triangle_area(ax, ay, bx, by, cx, cy);
    let area_abp = triangle_area(ax, ay, bx, by, px, py);
    let area_acp = triangle_area(ax, ay, cx, cy, px, py);
    let area_bcp = triangle_area(bx, by, cx, cy, px, py);

    // Check if the sum of area_abp, area_acp, and area_bcp is equal to area_abc
    (area_abp + area_acp + area_bcp - area_abc).abs() < f64::EPSILON
}

fn triangle_area(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)) / 2.0
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<f64> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if values.len() == 8 {
            let (ax, ay, bx, by, cx, cy, px, py) = (values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7]);
            if is_point_in_triangle(px, py, ax, ay, bx, by, cx, cy) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
