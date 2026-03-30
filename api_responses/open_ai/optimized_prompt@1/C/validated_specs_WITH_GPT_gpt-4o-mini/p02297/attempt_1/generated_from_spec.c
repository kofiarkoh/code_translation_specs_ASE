#include <stdio.h>

#define MAX_VERTICES 100 // Assuming a maximum number of vertices

// Function to calculate the area of a polygon using the shoelace formula
double calculate_area(int vertices[][2], int num_vertices) {
    double area = 0.0;
    
    // Shoelace formula implementation
    for (int i = 0; i < num_vertices; i++) {
        int j = (i + 1) % num_vertices; // Wrap around to the first vertex
        area += (vertices[i][0] * vertices[j][1]) - (vertices[j][0] * vertices[i][1]);
    }
    
    return area < 0 ? -area : area; // Ensure area is non-negative
}

int main() {
    int num_vertices;
    int vertices[MAX_VERTICES][2]; // Array to store vertex coordinates

    // Read the number of vertices
    scanf("%d", &num_vertices);
    
    // Read the coordinates of each vertex
    for (int i = 0; i < num_vertices; i++) {
        scanf("%d %d", &vertices[i][0], &vertices[i][1]);
    }

    // Calculate the area
    double area = calculate_area(vertices, num_vertices);
    
    // Output the area formatted to one decimal place
    printf("%.1f\n", area);
    
    return 0;
}
