#include <stdio.h>
#include <string.h>

#define MAX_NAMES 50
#define MAX_NAME_LENGTH 100

int main() {
    int N; // Number of names
    int Q; // Year threshold
    int years[MAX_NAMES]; // Array to store years
    char names[MAX_NAMES][MAX_NAME_LENGTH]; // Array to store names
    char *result_name; // Pointer to store the result name
    int most_recent_year = -1; // Variable to track the most recent valid year

    // Read the number of names and the year threshold
    scanf("%d", &N); // Input for N
    scanf("%d", &Q); // Input for Q

    // Preconditions check (undefined behavior if violated)
    if (N < 0 || N > MAX_NAMES || Q < 0) {
        return 1; // Exit with an error code for undefined behavior
    }

    // Read N pairs of years and names
    for (int i = 0; i < N; i++) {
        scanf("%d", &years[i]); // Read year
        scanf("%s", names[i]); // Read name
    }

    // Initialize result_name to the first name in case no valid year is found
    result_name = names[0];

    // Process each year and name
    for (int i = 0; i < N; i++) {
        // Check if the year is valid (less than or equal to Q)
        if (years[i] <= Q) {
            // Update the most recent year and corresponding name if necessary
            if (years[i] > most_recent_year) {
                most_recent_year = years[i];
                result_name = names[i]; // Update result_name to the current name
            }
        }
    }

    // Output the result name
    printf("%s\n", result_name);

    return 0; // Successful execution
}
