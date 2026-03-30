#include <stdio.h>

int main() {
    int X, Y;

    // Continuously process input until termination condition is met
    while (1) {
        // Read dimensions
        scanf("%d %d", &X, &Y);

        // Termination condition
        if (X == 0 && Y == 0) {
            break; // Exit the loop if both dimensions are zero
        }

        // Initialize the course data grid
        int courseData[X][Y];
        int sum = 0;

        // Read the course data into the grid
        for (int i = 0; i < X; i++) {
            for (int j = 0; j < Y; j++) {
                scanf("%d", &courseData[i][j]);
            }
        }

        // Analyze the course data based on specified rules
        for (int i = 0; i < X; i++) {
            for (int j = 0; j < Y; j++) {
                if (courseData[i][j] == 0) {
                    // Contribution from neighboring cells (not defined in the spec)
                    // Assuming a simple contribution from the cell itself
                    sum += 1; // Increment sum for each zero found
                } else if (courseData[i][j] == 1) {
                    // Reset contribution from this path to zero
                    sum = 0; // Reset sum if a 1 is encountered
                } else if (courseData[i][j] == 2) {
                    // Allow for additional contributions based on subsequent cells
                    // Assuming a simple increment for each two found
                    sum += 2; // Increment sum for each two found
                }
            }
        }

        // Output the computed sum
        printf("%d\n", sum);
    }

    return 0;
}
