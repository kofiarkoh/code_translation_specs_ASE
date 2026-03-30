#include <stdio.h>
#include <stdlib.h>

int minSubarrayLength(int N, int K, int* arr) {
    // If N is less than K, it's impossible to find a valid subarray
    if (N < K) {
        return 0;
    }

    int count[K + 1]; // Count occurrences of integers from 1 to K
    for (int i = 0; i <= K; i++) {
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
            count[arr[right]]++; // Increment count for this integer
        }
        right++; // Move the right pointer

        // Contract the window from the left as long as we have all integers from 1 to K
        while (uniqueCount == K) {
            // Update minLength if the current window is smaller
            if (right - left < minLength) {
                minLength = right - left;
            }

            // Try to remove arr[left] from the window
            if (arr[left] >= 1 && arr[left] <= K) {
                count[arr[left]]--; // Decrement count for this integer
                if (count[arr[left]] == 0) {
                    uniqueCount--; // We lost a unique integer
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
    int N = 7;
    int K = 3;
    int arr[] = {1, 2, 1, 3, 2, 1, 3};
    int result = minSubarrayLength(N, K, arr);
    printf("The length of the smallest subarray is: %d\n", result);
    return 0;
}
