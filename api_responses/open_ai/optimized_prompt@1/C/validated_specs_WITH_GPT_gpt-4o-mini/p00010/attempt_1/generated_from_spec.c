#include <stdio.h>

void compute_circumcenter_and_radius(float x1, float y1, float x2, float y2, float x3, float y3, float *cx, float *cy, float *r) {
    // Calculate the midpoints of the sides of the triangle
    float D = 2 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
    
    // Using the determinant method to find circumcenter coordinates
    *cx = ((x1 * x1 + y1 * y1) * (y2 - y3) + (x2 * x2 + y2 * y2) * (y3 - y1) + (x3 * x3 + y3 * y3) * (y1 - y2)) / D;
    *cy = ((x1 * x1 + y1 * y1) * (x3 - x2) + (x2 * x2 + y2 * y2) * (x1 - x3) + (x3 * x3 + y3 * y3) * (x2 - x1)) / D;

    // Calculate the circumradius using the distance from the circumcenter to one of the vertices
    *r = ((x1 - *cx) * (x1 - *cx) + (y1 - *cy) * (y1 - *cy)); // r^2
    *r = sqrt(*r); // r = sqrt(r^2)
}

int main() {
    int n;
    // Read the number of triangle vertex sets
    scanf("%d", &n);
    
    for (int i = 0; i < n; i++) {
        float x1, y1, x2, y2, x3, y3;
        // Read the coordinates of the triangle vertices
        scanf("%f %f %f %f %f %f", &x1, &y1, &x2, &y2, &x3, &y3);
        
        float cx, cy, r;
        // Compute the circumcenter and circumradius
        compute_circumcenter_and_radius(x1, y1, x2, y2, x3, y3, &cx, &cy, &r);
        
        // Output the results formatted to three decimal places
        printf("%.3f %.3f %.3f\n", cx, cy, r);
    }
    
    return 0;
}
