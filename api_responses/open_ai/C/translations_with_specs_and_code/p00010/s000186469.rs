use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_sets: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..num_sets {
        let mut points = String::new();
        io::stdin().read_line(&mut points).expect("Failed to read line");
        let coords: Vec<f64> = points
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid coordinate"))
            .collect();

        let (x1, y1, x2, y2, x3, y3) = (coords[0], coords[1], coords[2], coords[3], coords[4], coords[5]);

        let midx1 = (x1 + x2) / 2.0;
        let midy1 = (y1 + y2) / 2.0;
        let midx2 = (x1 + x3) / 2.0;
        let midy2 = (y1 + y3) / 2.0;

        let slant1 = -(x2 - x1) / (y2 - y1);
        let slant2 = -(x3 - x1) / (y3 - y1);
        let slice1 = midy1 - slant1 * midx1;
        let slice2 = midy2 - slant2 * midx2;

        let circlex = (slice2 - slice1) / (slant1 - slant2);
        let circley = slant1 * circlex + slice1;

        let (circlex, circley) = if x2 == x1 {
            let circley = midy1;
            let circlex = (circley - slice2) / slant2;
            (circlex, circley)
        } else if y2 == y1 {
            let circlex = midx1;
            let circley = slant2 * circlex + slice2;
            (circlex, circley)
        } else if x3 == x1 {
            let circley = midy2;
            let circlex = (circley - slice1) / slant1;
            (circlex, circley)
        } else if y3 == y1 {
            let circlex = midx2;
            let circley = slant1 * circlex + slice1;
            (circlex, circley)
        } else if x2 == x1 && y3 == y1 {
            (midx2, midy1)
        } else if x3 == x1 && y2 == y1 {
            (midx1, midy2)
        } else {
            (circlex, circley)
        };

        let radius = ((circlex - x1).powi(2) + (circley - y1).powi(2)).sqrt();
        println!("{:.3} {:.3} {:.3}", circlex, circley, radius);
    }
}
