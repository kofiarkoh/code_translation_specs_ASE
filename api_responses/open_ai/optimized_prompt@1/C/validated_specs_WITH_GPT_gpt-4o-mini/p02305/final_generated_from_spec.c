#include <stdio.h>

int determine_circle_relationship(int x1, int y1, int r1, int x2, int y2, int r2) {
    // Calculate the squared distance between the centers of the circles
    int dx = x2 - x1;
    int dy = y2 - y1;
    int distance_squared = dx * dx + dy * dy;

    // Calculate the squared radii
    int r1_plus_r2_squared = (r1 + r2) * (r1 + r2);
    int r1_minus_r2_squared = (r1 - r2) * (r1 - r2);

    // Determine the relationship based on the distances and radii
    if (distance_squared > r1_plus_r2_squared) {
        return 4; // The circles are separate
    } else if (distance_squared == r1_plus_r2_squared) {
        return 3; // The circles are tangent externally
    } else if (distance_squared < r1_minus_r2_squared) {
        return 0; // One circle is completely within the other without touching
    } else if (distance_squared == r1_minus_r2_squared) {
        return 1; // The circles are tangent internally
    } else {
        return 2; // The circles intersect
    }
}

int main() {
    int x1, y1, r1; // Circle 1: center (x1, y1) and radius r1
    int x2, y2, r2; // Circle 2: center (x2, y2) and radius r2

    // Read input values for the two circles
    scanf("%d %d %d", &x1, &y1, &r1);
    scanf("%d %d %d", &x2, &y2, &r2);

    // Call the function to determine the relationship and print the result
    int result = determine_circle_relationship(x1, y1, r1, x2, y2, r2);
    printf("%d\n", result);

    return 0;
}
