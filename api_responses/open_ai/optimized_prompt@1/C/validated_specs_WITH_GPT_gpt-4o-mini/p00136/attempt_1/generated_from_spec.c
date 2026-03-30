#include <stdio.h>

#define RANGE1_MAX 150.0
#define RANGE2_MAX 160.0
#define RANGE3_MAX 170.0
#define RANGE4_MAX 180.0
#define RANGE5_MAX 190.0
#define RANGE6_MAX 200.0

int main() {
    int num_measurements;
    // Read the number of height measurements
    scanf("%d", &num_measurements);

    // Preconditions: num_measurements must be non-negative
    if (num_measurements < 0) {
        return 1; // Exit if precondition is violated
    }

    // Initialize counts for each height range
    int count_range1 = 0;
    int count_range2 = 0;
    int count_range3 = 0;
    int count_range4 = 0;
    int count_range5 = 0;
    int count_range6 = 0;

    // Process each height measurement
    for (int i = 0; i < num_measurements; i++) {
        float height;
        scanf("%f", &height); // Read each height measurement

        // Count heights in the specified ranges
        if (height <= RANGE1_MAX) {
            count_range1++;
        } else if (height <= RANGE2_MAX) {
            count_range2++;
        } else if (height <= RANGE3_MAX) {
            count_range3++;
        } else if (height <= RANGE4_MAX) {
            count_range4++;
        } else if (height <= RANGE5_MAX) {
            count_range5++;
        } else if (height <= RANGE6_MAX) {
            count_range6++;
        }
        // Heights above RANGE6_MAX are ignored as per the specification
    }

    // Output the counts as asterisks
    printf("Range 1 (<= 150): ");
    for (int j = 0; j < count_range1; j++) {
        printf("*");
    }
    printf("\n");

    printf("Range 2 (<= 160): ");
    for (int j = 0; j < count_range2; j++) {
        printf("*");
    }
    printf("\n");

    printf("Range 3 (<= 170): ");
    for (int j = 0; j < count_range3; j++) {
        printf("*");
    }
    printf("\n");

    printf("Range 4 (<= 180): ");
    for (int j = 0; j < count_range4; j++) {
        printf("*");
    }
    printf("\n");

    printf("Range 5 (<= 190): ");
    for (int j = 0; j < count_range5; j++) {
        printf("*");
    }
    printf("\n");

    printf("Range 6 (<= 200): ");
    for (int j = 0; j < count_range6; j++) {
        printf("*");
    }
    printf("\n");

    return 0; // Successful execution
}
