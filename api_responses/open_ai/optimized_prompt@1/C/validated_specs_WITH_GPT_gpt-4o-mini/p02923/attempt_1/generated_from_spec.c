#include <stdio.h>

int longest_non_increasing_subsequence(int n, int arr[]) {
    // If n is zero, return zero as there are no elements to evaluate.
    if (n == 0) {
        return 0;
    }

    int max_length = 1; // Maximum length of non-increasing subsequence found
    int current_length = 1; // Current length of the non-increasing subsequence

    // Iterate through the array to find the longest non-increasing subsequence
    for (int i = 1; i < n; i++) {
        // Check if the current element is greater than or equal to the previous one
        if (arr[i] <= arr[i - 1]) {
            current_length++; // Increment current length
        } else {
            // Update max_length if current_length is greater
            if (current_length > max_length) {
                max_length = current_length;
            }
            current_length = 1; // Reset current length for new subsequence
        }
    }

    // Final check to update max_length after the loop
    if (current_length > max_length) {
        max_length = current_length;
    }

    return max_length; // Return the maximum length found
}

int main() {
    int n;
    // Read the number of elements
    scanf("%d", &n);
    
    int arr[n]; // Declare an array of size n

    // Read the sequence of integers
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }

    // Compute and print the length of the longest non-increasing contiguous subsequence
    printf("%d\n", longest_non_increasing_subsequence(n, arr));

    return 0;
}
