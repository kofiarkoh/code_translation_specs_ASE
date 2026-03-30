#include <stdio.h>

int longest_non_increasing_subsequence(int n, int arr[]) {
    // If there are no elements, return 0
    if (n == 0) {
        return 0;
    }

    int max_length = 1; // At least one element is present
    int current_length = 1; // Start with the first element

    // Iterate through the array to find the longest non-increasing subsequence
    for (int i = 1; i < n; i++) {
        // Check if the current element is less than or equal to the previous one
        if (arr[i] <= arr[i - 1]) {
            current_length++; // Increase the current non-increasing length
        } else {
            // Update max_length if the current sequence is longer
            if (current_length > max_length) {
                max_length = current_length;
            }
            current_length = 1; // Reset current length for a new sequence
        }
    }

    // Final check to update max_length in case the longest sequence ends at the last element
    if (current_length > max_length) {
        max_length = current_length;
    }

    return max_length; // Return the maximum length found
}

int main() {
    int n;
    // Read the number of elements
    scanf("%d", &n);
    
    // Declare an array to hold the sequence
    int arr[n];
    
    // Read the sequence of integers
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }

    // Compute and print the length of the longest non-increasing subsequence
    printf("%d\n", longest_non_increasing_subsequence(n, arr));
    
    return 0;
}
