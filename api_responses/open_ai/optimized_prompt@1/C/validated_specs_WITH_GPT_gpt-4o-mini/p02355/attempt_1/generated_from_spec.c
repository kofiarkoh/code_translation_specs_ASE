#include <stdio.h>
#include <stdlib.h>

int smallestSubarrayLength(int N, int K, int* arr) {
    // If N is less than K, it's impossible to find a valid subarray
    if (N < K) {
        return 0;
    }

    int count[K + 1]; // Count occurrences of integers from 1 to K
    for (int i = 1; i <= K; i++) {
        count[i] = 0; // Initialize counts to zero
    }

    int left = 0, right = 0; // Two pointers for the sliding window
    int uniqueCount = 0; // Count of unique integers from 1 to K in the current window
    int minLength = N + 1; // Initialize minLength to a value greater than any possible subarray length

    while (right < N) {
        // Expand the window by including arr[right]
        if (arr[right] >= 1 && arr[right] <= K) {
            if (count[arr[right]] == 0) {
                uniqueCount++; // New unique integer found
            }
            count[arr[right]]++; // Increment the count for this integer
        }
        right++; // Move the right pointer

        // Contract the window from the left as long as it contains all integers from 1 to K
        while (uniqueCount == K) {
            // Update minLength if the current window is smaller
            if (right - left < minLength) {
                minLength = right - left;
            }

            // Remove arr[left] from the window
            if (arr[left] >= 1 && arr[left] <= K) {
                count[arr[left]]--; // Decrement the count for this integer
                if (count[arr[left]] == 0) {
                    uniqueCount--; // One less unique integer in the window
                }
            }
            left++; // Move the left pointer
        }
    }

    // If minLength was updated, return it; otherwise, return 0
    return (minLength == N + 1) ? 0 : minLength;
}

// Example usage
int main() {
    int arr[] = {1, 2, 1, 3, 4, 2, 3};
    int N = sizeof(arr) / sizeof(arr[0]);
    int K = 4;
    int result = smallestSubarrayLength(N, K, arr);
    printf("Length of smallest subarray: %d\n", result);
    return 0;
}
