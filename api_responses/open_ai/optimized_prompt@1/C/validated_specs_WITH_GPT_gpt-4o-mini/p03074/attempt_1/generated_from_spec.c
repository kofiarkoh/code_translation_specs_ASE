#include <stdio.h>
#include <string.h>

#define MAX_LENGTH 1000 // Define a maximum limit for the input length

int max_segment_sum(int n, int k, const char *binary_string) {
    // Check preconditions
    if (n == 0 || strlen(binary_string) == 0) {
        return 0; // Return immediately if input size is zero
    }

    // Initialize variables
    int segment_counts[MAX_LENGTH] = {0}; // Array to hold counts of segments
    int segment_type[MAX_LENGTH] = {0};   // Array to hold type of segments (0 or 1)
    int segment_count = 0;                 // Number of segments found
    int current_count = 1;                 // Count of current segment

    // Process the binary string to create segments
    for (int i = 1; i < n; i++) {
        if (binary_string[i] == binary_string[i - 1]) {
            current_count++; // Increment count for the current segment
        } else {
            // Store the completed segment
            segment_counts[segment_count] = current_count;
            segment_type[segment_count] = binary_string[i - 1] - '0'; // Store segment type (0 or 1)
            segment_count++;
            current_count = 1; // Reset count for the new segment
        }
    }
    // Store the last segment
    segment_counts[segment_count] = current_count;
    segment_type[segment_count] = binary_string[n - 1] - '0';
    segment_count++;

    // If there are no segments, return 0
    if (segment_count == 0) {
        return 0;
    }

    // Calculate the maximum sum of counts of segments with allowed changes
    int max_sum = 0;

    // Iterate through segments and calculate sums with changes
    for (int i = 0; i < segment_count; i++) {
        int current_sum = segment_counts[i];
        int changes_left = k;

        // Check for segments to the right
        for (int j = i + 1; j < segment_count; j++) {
            if (segment_type[j] != segment_type[i]) {
                if (changes_left > 0) {
                    current_sum += segment_counts[j]; // Add the next segment
                    changes_left--; // Use a change
                } else {
                    break; // No more changes left
                }
            } else {
                current_sum += segment_counts[j]; // Add the same type segment
            }
        }

        // Update max_sum if current_sum is greater
        if (current_sum > max_sum) {
            max_sum = current_sum;
        }
    }

    return max_sum; // Return the maximum sum found
}

int main() {
    int n, k;
    char binary_string[MAX_LENGTH];

    // Example input
    n = 10; // Number of binary digits
    k = 1;  // Number of allowed changes
    strcpy(binary_string, "1100011000"); // Input binary string

    // Call the function and print the result
    int result = max_segment_sum(n, k, binary_string);
    printf("Maximum sum of segments: %d\n", result);

    return 0;
}
