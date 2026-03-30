#include <stdio.h>

int count_peaks_and_valleys(int n, int arr[]) {
    // Preconditions check: n must be >= 3
    if (n < 3) {
        // Undefined behavior as per specification; no handling implemented
        return -1; // Indicate error condition (not specified in the original)
    }

    int count = 0;
    int last_feature = 0; // 0 = none, 1 = peak, -1 = valley

    for (int i = 1; i < n - 1; i++) {
        // Check for peak
        if (arr[i] > arr[i - 1] && arr[i] > arr[i + 1]) {
            if (last_feature != 1) { // Only count if last was not a peak
                count++;
                last_feature = 1; // Update last feature to peak
            }
        }
        // Check for valley
        else if (arr[i] < arr[i - 1] && arr[i] < arr[i + 1]) {
            if (last_feature != -1) { // Only count if last was not a valley
                count++;
                last_feature = -1; // Update last feature to valley
            }
        }
    }

    return count; // Return the total count of distinct peaks and valleys
}

int main() {
    int n = 5; // Example size
    int arr[] = {1, 3, 2, 4, 3}; // Example sequence
    int result = count_peaks_and_valleys(n, arr);
    printf("Count of peaks and valleys: %d\n", result);
    return 0;
}
