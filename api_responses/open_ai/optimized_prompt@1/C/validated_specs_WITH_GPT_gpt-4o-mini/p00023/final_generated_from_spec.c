#include <stdio.h>

int circle_relationship(int n, float circles[][6]) {
    int results[n]; // Array to store results for each pair of circles

    for (int i = 0; i < n; i++) {
        float a_x = circles[i][0];
        float a_y = circles[i][1];
        float a_r = circles[i][2];
        float b_x = circles[i][3];
        float b_y = circles[i][4];
        float b_r = circles[i][5];

        // Calculate squared distance between the centers
        float dx = a_x - b_x;
        float dy = a_y - b_y;
        float dist_sq = dx * dx + dy * dy;

        // Calculate squared radii
        float r_sum_sq = (a_r + b_r) * (a_r + b_r);
        float r_diff_sq = (a_r - b_r) * (a_r - b_r);

        // Determine the relationship based on the distances and radii
        if (dist_sq > r_sum_sq) {
            results[i] = 0; // Circles do not intersect
        } else if (dist_sq == r_sum_sq) {
            results[i] = 1; // Circles intersect at one point (externally tangent)
        } else if (dist_sq < r_diff_sq) {
            if (a_r < b_r) {
                results[i] = 2; // Circle a is completely within circle b
            } else {
                results[i] = -2; // Circle b is completely within circle a
            }
        } else if (dist_sq == r_diff_sq) {
            results[i] = -2; // One circle is completely within the other but touching internally
        } else {
            results[i] = 1; // Circles intersect at one or more points
        }
    }

    // Output results
    for (int i = 0; i < n; i++) {
        printf("%d\n", results[i]);
    }

    return 0; // Indicate successful execution
}

int main() {
    // Example usage
    int n = 2; // Number of circle pairs
    float circles[2][6] = {
        {0.0, 0.0, 1.0, 2.0, 0.0, 1.0}, // Example pair 1
        {0.0, 0.0, 1.0, 0.5, 0.0, 0.5}  // Example pair 2
    };

    circle_relationship(n, circles); // Call the function with example data
    return 0; // Indicate successful execution
}
