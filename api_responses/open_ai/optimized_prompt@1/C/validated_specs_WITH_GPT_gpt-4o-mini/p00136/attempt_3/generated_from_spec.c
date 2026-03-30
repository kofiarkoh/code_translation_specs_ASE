#include <stdio.h>

#define RANGE_COUNT 6

int main() {
    int num_measurements;
    float height;
    int counts[RANGE_COUNT] = {0}; // Initialize counts for each range to zero

    // Read the number of height measurements
    scanf("%d", &num_measurements);

    // Process each height measurement
    for (int i = 0; i < num_measurements; i++) {
        scanf("%f", &height);
        
        // Categorize height into ranges
        if (height < 165.0) {
            counts[0]++;
        } else if (height < 170.0) {
            counts[1]++;
        } else if (height < 175.0) {
            counts[2]++;
        } else if (height < 180.0) {
            counts[3]++;
        } else if (height < 185.0) {
            counts[4]++;
        } else {
            counts[5]++;
        }
    }

    // Output the counts as asterisks
    for (int i = 0; i < RANGE_COUNT; i++) {
        printf("%d: ", i + 1); // Print range number
        for (int j = 0; j < counts[i]; j++) {
            putchar('*'); // Print asterisks for the count
        }
        putchar('\n'); // New line after each range
    }

    return 0;
}
