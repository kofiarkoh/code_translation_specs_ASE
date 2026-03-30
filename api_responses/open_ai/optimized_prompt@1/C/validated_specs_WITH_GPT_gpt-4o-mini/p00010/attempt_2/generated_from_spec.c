#include <stdio.h>
#include <math.h>

void compute_circumcenter_and_radius(float x1, float y1, float x2, float y2, float x3, float y3, float *cx, float *cy, float *r) {
    // Calculate the midpoints of the sides
    float D = 2 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
    
    // Circumcenter coordinates
    *cx = ((x1 * x1 + y1 * y1) * (y2 - y3) + (x2 * x2 + y2 * y2) * (y3 - y1) + (x3 * x3 + y3 * y3) * (y1 - y2)) / D;
    *cy = ((x1 * x1 + y1 * y1) * (x3 - x2) + (x2 * x2 + y2 * y2) * (x1 - x3) + (x3 * x3 + y3 * y3) * (x2 - x1)) / D;

    // Circumradius calculation
    float a = sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1));
    float b = sqrt((x3 - x2) * (x3 - x2) + (y3 - y2) * (y3 - y2));
    float c = sqrt((x1 - x3) * (x1 - x3) + (y1 - y3) * (y1 - y3));
    
    // Area of the triangle using Heron's formula
    float s = (a + b + c) / 2;
    float area = sqrt(s * (s - a) * (s - b) * (s - c));
    
    // Circumradius
    *r = (a * b * c) / (4 * area);
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
        // Compute circumcenter and circumradius
        compute_circumcenter_and_radius(x1, y1, x2, y2, x3, y3, &cx, &cy, &r);
        
        // Output the results formatted to three decimal places
        printf("%.3f %.3f %.3f\n", cx, cy, r);
    }
    
    return 0;
}
