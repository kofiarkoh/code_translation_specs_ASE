#include <stdio.h>
#include <string.h>

#define NUM_MAX 1000 // Define a maximum limit for num

int max_segment_sum(int num, int changes, const char *binary_string) {
    // Check preconditions
    if (num == 0 || strlen(binary_string) == 0) {
        return 0; // Return immediately if num is zero or string is empty
    }
    if (num > NUM_MAX) {
        return -1; // Error case: num exceeds maximum limit
    }

    int segment_counts[NUM_MAX]; // Array to hold counts of segments
    int segment_types[NUM_MAX];   // Array to hold types of segments (0 or 1)
    int segment_count = 0;        // Number of segments found

    // Process the binary string to create segments
    for (int i = 0; i < num; i++) {
        if (i == 0 || binary_string[i] != binary_string[i - 1]) {
            // New segment detected
            segment_types[segment_count] = binary_string[i] - '0'; // Store segment type (0 or 1)
            segment_counts[segment_count] = 1; // Initialize count for this segment
            segment_count++;
        } else {
            // Increment count for the current segment
            segment_counts[segment_count - 1]++;
        }
    }

    // If there are no segments, return 0
    if (segment_count == 0) {
        return 0;
    }

    // Calculate the maximum sum of segments with allowed changes
    int max_sum = 0;

    // Iterate through segments and calculate sums based on allowed changes
    for (int i = 0; i < segment_count; i++) {
        int current_sum = segment_counts[i]; // Start with the current segment count
        int changes_left = changes;

        // Try to combine with next segments if changes are available
        for (int j = i + 1; j < segment_count && changes_left > 0; j++) {
            current_sum += segment_counts[j]; // Add the next segment count
            changes_left--; // Use a change to combine segments
        }

        // Update max_sum if the current sum is greater
        if (current_sum > max_sum) {
            max_sum = current_sum;
        }
    }

    return max_sum; // Return the maximum sum found
}

int main() {
    int num, changes;
    char binary_string[NUM_MAX + 1]; // +1 for null terminator

    // Example input
    num = 10; // Number of binary digits
    changes = 1; // Number of allowed changes
    strcpy(binary_string, "1100011000"); // Example binary string

    int result = max_segment_sum(num, changes, binary_string);
    printf("Maximum sum of segments: %d\n", result);

    return 0;
}
