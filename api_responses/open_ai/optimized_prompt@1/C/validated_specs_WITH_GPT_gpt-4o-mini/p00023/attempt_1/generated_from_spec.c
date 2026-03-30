#include <stdio.h>
#include <math.h>

int determine_circle_relationship(float ax, float ay, float ar, float bx, float by, float br) {
    // Calculate the distance squared between the centers of the circles
    float dx = ax - bx;
    float dy = ay - by;
    float distance_squared = dx * dx + dy * dy;
    
    // Calculate the sum of the radii squared
    float radii_sum_squared = (ar + br) * (ar + br);
    // Calculate the difference of the radii squared
    float radii_diff_squared = (ar - br) * (ar - br);
    
    // Determine the relationship based on the distance and radii
    if (distance_squared > radii_sum_squared) {
        return 0; // Circles do not intersect
    } else if (distance_squared < radii_diff_squared) {
        // One circle is completely within the other
        if (ar < br) {
            return 2; // Circle a is completely within circle b
        } else {
            return -2; // Circle b is completely within circle a
        }
    } else {
        return 1; // Circles intersect
    }
}

int main() {
    int n;
    // Read the number of circle pairs
    scanf("%d", &n);
    
    // Process each pair of circles
    for (int i = 0; i < n; i++) {
        float ax, ay, ar, bx, by, br;
        // Read the parameters for the two circles
        scanf("%f %f %f %f %f %f", &ax, &ay, &ar, &bx, &by, &br);
        
        // Determine the relationship and print the result
        int result = determine_circle_relationship(ax, ay, ar, bx, by, br);
        printf("%d\n", result);
    }
    
    return 0;
}
