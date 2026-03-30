#include <stdio.h>

#define MAX_VERTICES 100 // Assuming a maximum number of vertices for the polygon

// Function to calculate the area of a polygon using the shoelace formula
long calculate_polygon_area(int vertices[][2], int num_vertices) {
    long area = 0;
    for (int i = 0; i < num_vertices; i++) {
        int j = (i + 1) % num_vertices; // Wrap around to the first vertex
        area += vertices[i][0] * vertices[j][1];
        area -= vertices[j][0] * vertices[i][1];
    }
    return area / 2; // The area is half the absolute value of the computed sum
}

// Function to calculate the area of the bounding box
long calculate_bounding_box_area(int box[4][2]) {
    int width = box[1][0] - box[0][0]; // Assuming box[0] is bottom-left and box[1] is top-right
    int height = box[2][1] - box[0][1]; // Assuming box[0] is bottom-left and box[2] is top-left
    return (long)width * height; // Return area as long
}

int main() {
    int vertices[MAX_VERTICES][2]; // Array to store polygon vertices
    int bounding_box[4][2]; // Array to store bounding box coordinates
    int num_vertices;

    // Read input until termination condition (EOF)
    while (scanf("%d", &num_vertices) == 1 && num_vertices > 0) {
        // Read polygon vertices
        for (int i = 0; i < num_vertices; i++) {
            scanf("%d %d", &vertices[i][0], &vertices[i][1]);
        }

        // Read bounding box coordinates
        for (int i = 0; i < 4; i++) {
            scanf("%d %d", &bounding_box[i][0], &bounding_box[i][1]);
        }

        // Calculate areas
        long polygon_area = calculate_polygon_area(vertices, num_vertices);
        long bounding_box_area = calculate_bounding_box_area(bounding_box);

        // Calculate area difference
        long area_difference = polygon_area - bounding_box_area;

        // Output the area difference
        printf("%ld\n", area_difference);
    }

    return 0;
}
