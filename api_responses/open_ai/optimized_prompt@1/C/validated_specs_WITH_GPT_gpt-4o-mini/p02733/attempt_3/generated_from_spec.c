#include <stdio.h>

int min_segments(int H, int W, int K, char S[H][W]) {
    int segments = 0; // To count the number of segments
    int start_row = 0; // Starting row of the current segment

    while (start_row < H) {
        int end_row = start_row; // Initialize end_row to start_row
        int count[W]; // Array to count '1's in each column
        for (int j = 0; j < W; j++) {
            count[j] = 0; // Initialize column counts to 0
        }

        // Expand the segment until the count of '1's exceeds K
        while (end_row < H) {
            // Update counts for the current end_row
            for (int j = 0; j < W; j++) {
                if (S[end_row][j] == '1') {
                    count[j]++;
                }
            }

            // Check if any column exceeds the limit K
            int exceeds = 0;
            for (int j = 0; j < W; j++) {
                if (count[j] > K) {
                    exceeds = 1; // Mark that we exceeded the limit
                    break;
                }
            }

            if (exceeds) {
                break; // Stop expanding if we exceeded K
            }

            end_row++; // Move to the next row
        }

        // We have found a valid segment from start_row to end_row - 1
        segments++; // Increment the segment count
        start_row = end_row; // Move to the next segment starting row
    }

    return segments; // Return the total number of segments found
}

int main() {
    // Example usage
    int H = 4, W = 3, K = 2;
    char S[4][3] = {
        {'1', '0', '1'},
        {'1', '1', '0'},
        {'0', '1', '1'},
        {'1', '0', '0'}
    };

    int result = min_segments(H, W, K, S);
    printf("%d\n", result); // Output the result
    return 0;
}
