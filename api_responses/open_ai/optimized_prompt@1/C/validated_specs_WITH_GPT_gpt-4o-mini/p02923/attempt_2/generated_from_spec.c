#include <stdio.h>

int longest_non_increasing_subsequence(int n, int arr[]) {
    // Handle the case where there are no elements
    if (n == 0) {
        return 0; // No elements to evaluate
    }

    int max_length = 1; // At least one element is present
    int current_length = 1; // Start with the first element

    // Iterate through the array to find the longest non-increasing subsequence
    for (int i = 1; i < n; i++) {
        if (arr[i] <= arr[i - 1]) {
            // Current element is non-increasing compared to the previous one
            current_length++;
        } else {
            // Reset current length if the sequence increases
            if (current_length > max_length) {
                max_length = current_length; // Update max length if needed
            }
            current_length = 1; // Reset for the new subsequence
        }
    }

    // Final check to update max_length in case the longest subsequence ends at the last element
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
