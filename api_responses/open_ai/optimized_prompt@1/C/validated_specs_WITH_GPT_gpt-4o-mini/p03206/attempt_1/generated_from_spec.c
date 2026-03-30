#include <stdio.h>

int main() {
    int day; // Variable to store the input day relative to Christmas

    // Read an integer input
    if (scanf("%d", &day) != 1) {
        // If input is not an integer, do nothing (as per specification)
        return 0;
    }

    // Output the corresponding holiday string based on the input value
    if (day == 25) {
        printf("Christmas\n");
    } else if (day == 24) {
        printf("Christmas Eve\n");
    } else if (day == 23) {
        printf("Christmas Eve Eve\n");
    } else if (day == 22) {
        printf("Christmas Eve Eve Eve\n");
    }
    // No output for any other integer values, as specified

    return 0; // Indicate successful completion
}
