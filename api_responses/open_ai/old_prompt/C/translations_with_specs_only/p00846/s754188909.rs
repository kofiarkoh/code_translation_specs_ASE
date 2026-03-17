// Define a structure to represent a point in 2D space
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Function to check if a point is inside a square defined by its bottom-left and top-right corners
fn is_point_in_square(point: &Point, square_bottom_left: &Point, square_top_right: &Point) -> bool {
    point.x >= square_bottom_left.x && point.x <= square_top_right.x &&
    point.y >= square_bottom_left.y && point.y <= square_top_right.y
}

// Function to check if two line segments intersect
fn do_lines_intersect(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
    let orientation = |p: &Point, q: &Point, r: &Point| {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0 { 0 } // collinear
        else if val > 0 { 1 } // clockwise
        else { 2 } // counterclockwise
    };

    let o1 = orientation(p1, p2, p3);
    let o2 = orientation(p1, p2, p4);
    let o3 = orientation(p3, p4, p1);
    let o4 = orientation(p3, p4, p2);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special cases (collinear points)
    // Check if p3 lies on segment p1p2
    if o1 == 0 && is_point_on_segment(p1, p2, p3) { return true; }
    // Check if p4 lies on segment p1p2
    if o2 == 0 && is_point_on_segment(p1, p2, p4) { return true; }
    // Check if p1 lies on segment p3p4
    if o3 == 0 && is_point_on_segment(p3, p4, p1) { return true; }
    // Check if p2 lies on segment p3p4
    if o4 == 0 && is_point_on_segment(p3, p4, p2) { return true; }

    false
}

// Function to check if a point lies on a segment
fn is_point_on_segment(p1: &Point, p2: &Point, p: &Point) -> bool {
    p.x <= p2.x.max(p1.x) && p.x >= p2.x.min(p1.x) &&
    p.y <= p2.y.max(p1.y) && p.y >= p2.y.min(p1.y)
}

// Function to check if a polygon intersects with a square
fn does_polygon_intersect_square(vertices: &[Point], n: usize, square_bottom_left: &Point, square_top_right: &Point) -> bool {
    // Check if any vertex of the polygon is inside the square
    for i in 0..n {
        if is_point_in_square(&vertices[i], square_bottom_left, square_top_right) {
            return true;
        }
    }

    // Check for intersections between polygon edges and square edges
    let square_vertices = [
        *square_bottom_left,
        Point { x: square_top_right.x, y: square_bottom_left.y },
        *square_top_right,
        Point { x: square_bottom_left.x, y: square_top_right.y },
    ];

    for i in 0..n {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % n]; // Wrap around to the first vertex

        for j in 0..4 {
            let p3 = square_vertices[j];
            let p4 = square_vertices[(j + 1) % 4]; // Wrap around to the first square vertex

            if do_lines_intersect(&p1, &p2, &p3, &p4) {
                return true;
            }
        }
    }

    false
}

// Main function to read input and output the result
fn main() {
    let n: usize = 4; // Example number of vertices
    let vertices = vec![
        Point { x: 1, y: 1 },
        Point { x: 4, y: 1 },
        Point { x: 4, y: 4 },
        Point { x: 1, y: 4 },
    ]; // Example vertices of a polygon

    let square_bottom_left = Point { x: 2, y: 2 };
    let square_top_right = Point { x: 3, y: 3 };

    // Check for intersection and print result
    if does_polygon_intersect_square(&vertices, n, &square_bottom_left, &square_top_right) {
        println!("1");
    } else {
        println!("0");
    }
}
