#include <stdio.h>

#define MAX_VERTICES 50

// Structure to represent a point in 2D space
typedef struct {
    int x;
    int y;
} Point;

// Predefined convex shape (example: a square)
Point convex_shape[] = {
    {0, 0},
    {0, 10},
    {10, 10},
    {10, 0}
};
int convex_shape_size = 4;

// Function to calculate the cross product of vectors OA and OB
// A positive cross product indicates a counter-clockwise turn, 
// a negative indicates a clockwise turn, and zero indicates collinearity.
int cross_product(Point O, Point A, Point B) {
    return (A.x - O.x) * (B.y - O.y) - (A.y - O.y) * (B.x - O.x);
}

// Function to check if two line segments AB and CD intersect
int do_intersect(Point A, Point B, Point C, Point D) {
    int d1 = cross_product(A, B, C);
    int d2 = cross_product(A, B, D);
    int d3 = cross_product(C, D, A);
    int d4 = cross_product(C, D, B);

    // General case
    if (d1 * d2 < 0 && d3 * d4 < 0) {
        return 1; // They intersect
    }

    // Special cases for collinearity
    if (d1 == 0 && (C.x >= A.x && C.x <= B.x || C.x <= A.x && C.x >= B.x)) return 1;
    if (d2 == 0 && (D.x >= A.x && D.x <= B.x || D.x <= A.x && D.x >= B.x)) return 1;
    if (d3 == 0 && (A.x >= C.x && A.x <= D.x || A.x <= C.x && A.x >= D.x)) return 1;
    if (d4 == 0 && (B.x >= C.x && B.x <= D.x || B.x <= C.x && B.x >= D.x)) return 1;

    return 0; // No intersection
}

// Main function to read polygon vertices and check for intersection
int main() {
    int n;
    Point polygon[MAX_VERTICES];

    // Read the number of vertices
    scanf("%d", &n);
    
    // Check for valid input
    if (n <= 0) {
        return 0; // Terminate if n is zero or negative
    }

    // Read the vertices of the polygon
    for (int i = 0; i < n; i++) {
        scanf("%d %d", &polygon[i].x, &polygon[i].y);
    }

    // Check for intersections between polygon edges and convex shape edges
    for (int i = 0; i < n; i++) {
        Point A = polygon[i];
        Point B = polygon[(i + 1) % n]; // Wrap around to the first vertex
        for (int j = 0; j < convex_shape_size; j++) {
            Point C = convex_shape[j];
            Point D = convex_shape[(j + 1) % convex_shape_size]; // Wrap around to the first vertex
            if (do_intersect(A, B, C, D)) {
                printf("1\n"); // Intersection found
                return 0; // Exit after finding the first intersection
            }
        }
    }

    printf("0\n"); // No intersection found
    return 0;
}
