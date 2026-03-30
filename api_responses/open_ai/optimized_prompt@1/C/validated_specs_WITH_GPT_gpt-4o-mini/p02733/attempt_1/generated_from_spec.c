#include <stdio.h>

int min_segments(int H, int W, int K, char S[H][W]) {
    int segments = 0; // To count the number of segments
    int start_row = 0; // Starting row of the current segment

    // Iterate through each column
    for (int col = 0; col < W; col++) {
        int count_ones = 0; // Count of '1's in the current segment

        // Iterate through rows to find segments
        for (int row = 0; row < H; row++) {
            // Count '1's in the current row for the current column
            if (S[row][col] == '1') {
                count_ones++;
            }

            // If count exceeds K, we need to finalize the current segment
            if (count_ones > K) {
                segments++; // Increment the segment count
                start_row = row; // Start a new segment from the current row
                count_ones = 1; // Reset count for the new segment, current row is '1'
            }
        }

        // If there were any rows processed, we need to account for the last segment
        if (count_ones > 0) {
            segments++; // Final segment for the current column
        }
    }

    return segments; // Return the total number of segments
}

int main() {
    // Example usage
    int H = 3, W = 3, K = 1;
    char S[3][3] = {
        {'1', '0', '1'},
        {'0', '1', '0'},
        {'1', '1', '1'}
    };

    int result = min_segments(H, W, K, S);
    printf("%d\n", result); // Output the result
    return 0;
}
