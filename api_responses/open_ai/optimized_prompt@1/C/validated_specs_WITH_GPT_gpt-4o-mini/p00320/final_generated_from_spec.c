#include <stdio.h>
#include <stdlib.h>

#define COMPONENTS 6

// Function to swap two integers
void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

// Function to generate permutations and check for valid configuration
int check_permutations(int dimensions[COMPONENTS][2], int indices[COMPONENTS]) {
    // Placeholder for connection rules evaluation
    // This function should implement the specific connection rules
    // For demonstration, we assume a simple condition for validity
    for (int i = 0; i < COMPONENTS; i++) {
        // Example condition: sum of heights must equal sum of widths
        if (dimensions[indices[i]][0] != dimensions[indices[i]][1]) {
            return 0; // Invalid configuration
        }
    }
    return 1; // Valid configuration found
}

// Recursive function to generate permutations
void permute(int dimensions[COMPONENTS][2], int indices[COMPONENTS], int start, int *found) {
    if (start == COMPONENTS) {
        if (check_permutations(dimensions, indices)) {
            *found = 1; // Valid configuration found
        }
        return;
    }
    for (int i = start; i < COMPONENTS; i++) {
        swap(&indices[start], &indices[i]);
        permute(dimensions, indices, start + 1, found);
        swap(&indices[start], &indices[i]); // backtrack
    }
}

int main() {
    int dimensions[COMPONENTS][2];
    int indices[COMPONENTS] = {0, 1, 2, 3, 4, 5}; // Initialize indices for permutations
    int found = 0;

    // Read input
    for (int i = 0; i < COMPONENTS; i++) {
        if (scanf("%d %d", &dimensions[i][0], &dimensions[i][1]) != 2 || dimensions[i][0] < 0 || dimensions[i][1] < 0) {
            return 1; // Terminate if input is invalid
        }
    }

    // Generate permutations and check for valid configuration
    permute(dimensions, indices, 0, &found);

    // Output result
    if (found) {
        printf("yes\n");
    } else {
        printf("no\n");
    }

    return 0;
}
