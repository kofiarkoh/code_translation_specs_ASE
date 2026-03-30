#include <stdio.h>
#include <stdlib.h>

void process_chair_identifiers(int n, int *identifiers, int *output) {
    // Array to store the last occurrence index of each identifier
    int *last_occurrence = (int *)malloc((n + 1) * sizeof(int));
    
    // Initialize last_occurrence array to -1 (indicating not found)
    for (int i = 0; i <= n; i++) {
        last_occurrence[i] = -1;
    }

    // Record the last occurrence of each identifier
    for (int i = 0; i < n; i++) {
        last_occurrence[identifiers[i]] = i; // Store the index of the last occurrence
    }

    // Generate the output based on the last occurrence indices
    for (int i = 0; i < n; i++) {
        output[i] = last_occurrence[identifiers[i]]; // Get the last occurrence index
        if (output[i] == -1) {
            output[i] = 0; // If not found, set to 0 as per specification
        }
    }

    // Free the allocated memory for last_occurrence
    free(last_occurrence);
}

int main() {
    int n;
    scanf("%d", &n); // Read the number of identifiers

    int *identifiers = (int *)malloc(n * sizeof(int));
    int *output = (int *)malloc(n * sizeof(int));

    // Read the identifiers
    for (int i = 0; i < n; i++) {
        scanf("%d", &identifiers[i]);
    }

    // Process the identifiers to generate the output
    process_chair_identifiers(n, identifiers, output);

    // Print the output sequence
    for (int i = 0; i < n; i++) {
        printf("%d ", output[i]);
    }
    printf("\n");

    // Free allocated memory
    free(identifiers);
    free(output);

    return 0;
}
