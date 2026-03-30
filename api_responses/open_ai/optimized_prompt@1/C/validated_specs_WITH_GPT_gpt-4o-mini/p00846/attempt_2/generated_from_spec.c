#include <stdio.h>

#define MAX_VERTICES 50

// Predefined convex shape vertices (example: a square)
int convex_shape[][2] = {
    {0, 0},
    {0, 10},
    {10, 10},
    {10, 0}
};
const int convex_shape_size = 4;

// Function to check if two line segments intersect
int do_intersect(int p1[], int q1[], int p2[], int q2[]) {
    // Function to calculate the orientation of the ordered triplet (p, q, r)
    int orientation(int p[], int q[], int r[]) {
        int val = (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1]);
        if (val == 0) return 0;  // collinear
        return (val > 0) ? 1 : 2; // clock or counterclockwise
    }

    // Check the orientation of the points
    int o1 = orientation(p1, q1, p2);
    int o2 = orientation(p1, q1, q2);
    int o3 = orientation(p2, q2, p1);
    int o4 = orientation(p2, q2, q1);

    // General case
    if (o1 != o2 && o3 != o4) return 1;

    // Special cases (collinear points)
    // Check if the points are on the segment
    // (omitted for brevity, but would check if points are on the segments)

    return 0; // No intersection
}

// Function to check if the polygon intersects with the convex shape
int polygon_intersects(int polygon[][2], int num_vertices) {
    for (int i = 0; i < num_vertices; i++) {
        int p1[] = {polygon[i][0], polygon[i][1]};
        int q1[] = {polygon[(i + 1) % num_vertices][0], polygon[(i + 1) % num_vertices][1]};
        
        for (int j = 0; j < convex_shape_size; j++) {
            int p2[] = {convex_shape[j][0], convex_shape[j][1]};
            int q2[] = {convex_shape[(j + 1) % convex_shape_size][0], convex_shape[(j + 1) % convex_shape_size][1]};
            
            if (do_intersect(p1, q1, p2, q2)) {
                return 1; // Intersection found
            }
        }
    }
    return 0; // No intersection
}

int main() {
    int num_vertices;
    int polygon[MAX_VERTICES][2];

    // Read the number of vertices
    scanf("%d", &num_vertices);
    
    // Check for valid number of vertices
    if (num_vertices <= 0) {
        return 0; // Terminate if invalid input
    }

    // Read the polygon vertices
    for (int i = 0; i < num_vertices; i++) {
        scanf("%d %d", &polygon[i][0], &polygon[i][1]);
    }

    // Check for intersection and print result
    int result = polygon_intersects(polygon, num_vertices);
    printf("%d\n", result);

    return 0;
}
