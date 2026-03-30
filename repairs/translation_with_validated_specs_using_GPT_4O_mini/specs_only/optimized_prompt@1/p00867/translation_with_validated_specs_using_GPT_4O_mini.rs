#[derive(Debug)]
struct Segment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Segment {
    // Check if two segments are collinear
    fn is_collinear(&self, other: &Segment) -> bool {
        let area = (self.x2 - self.x1) * (other.y1 - self.y1) - (self.y2 - self.y1) * (other.x1 - self.x1);
        area == 0
    }

    // Check if two segments overlap at endpoints
    fn overlaps(&self, other: &Segment) -> bool {
        (self.x1 == other.x1 && self.y1 == other.y1) || (self.x1 == other.x2 && self.y1 == other.y2) ||
        (self.x2 == other.x1 && self.y2 == other.y1) || (self.x2 == other.x2 && self.y2 == other.y2)
    }

    // Check if two segments intersect at points other than endpoints
    fn intersects(&self, other: &Segment) -> bool {
        let d1 = self.direction(other.x1, other.y1);
        let d2 = self.direction(other.x2, other.y2);
        let d3 = other.direction(self.x1, self.y1);
        let d4 = other.direction(self.x2, self.y2);
        (d1 != d2 && d3 != d4)
    }

    // Calculate direction for intersection check
    fn direction(&self, x: i32, y: i32) -> i32 {
        (self.x2 - self.x1) * (y - self.y1) - (self.y2 - self.y1) * (x - self.x1)
    }

    // Check if a point is within the segment bounds
    fn contains(&self, x: i32, y: i32) -> bool {
        let min_x = self.x1.min(self.x2);
        let max_x = self.x1.max(self.x2);
        let min_y = self.y1.min(self.y2);
        let max_y = self.y1.max(self.y2);
        (x >= min_x && x <= max_x) && (y >= min_y && y <= max_y)
    }
}

fn count_configurations(segments: Vec<Segment>) -> [i32; 10] {
    let mut counts = [0; 10];

    for i in 0..segments.len() {
        for j in (i + 1)..segments.len() {
            if segments[i].is_collinear(&segments[j]) {
                if segments[i].overlaps(&segments[j]) {
                    counts[0] += 1; // Count collinear overlaps
                } else {
                    counts[1] += 1; // Count collinear non-overlaps
                }
            } else if segments[i].intersects(&segments[j]) {
                counts[2] += 1; // Count distinct intersections
            }
        }
    }

    // Update counts based on the specific conditions defined in the specification
    counts[6] = counts[0] + counts[1]; // Total collinear segments
    // Other counts can be updated based on additional criteria if needed

    counts
}

fn main() {
    // Example input: number of segments followed by their coordinates
    let input = vec![
        Segment { x1: 0, y1: 0, x2: 1, y2: 1 },
        Segment { x1: 1, y1: 1, x2: 2, y2: 2 },
        Segment { x1: 2, y1: 2, x2: 3, y2: 3 },
        Segment { x1: 3, y1: 3, x2: 4, y2: 4 },
        Segment { x1: 4, y1: 4, x2: 5, y2: 5 },
    ];

    let counts = count_configurations(input);
    println!("{:?}", counts);
}
