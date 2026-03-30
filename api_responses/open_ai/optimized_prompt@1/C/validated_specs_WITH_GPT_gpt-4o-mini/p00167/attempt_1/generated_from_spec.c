#include <stdio.h>

#define MAX_SIZE 100

// Function to perform bubble sort and count swaps
int bubble_sort(int arr[], int n) {
    int swap_count = 0;
    for (int i = 0; i < n - 1; i++) {
        for (int j = 0; j < n - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                // Swap arr[j] and arr[j + 1]
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                swap_count++; // Increment swap count
            }
        }
    }
    return swap_count; // Return total number of swaps
}

int main() {
    int n; // Number of integers to sort
    while (1) {
        // Read the first integer
        scanf("%d", &n);
        
        // Termination condition
        if (n == 0) {
            break; // Exit the loop if the first integer is zero
        }
        
        // Preconditions check
        if (n < 0 || n > MAX_SIZE) {
            // Undefined behavior as per specification; we do not handle this case
            continue; // Skip to the next iteration
        }

        int arr[MAX_SIZE]; // Array to hold the integers
        
        // Read the subsequent integers
        for (int i = 0; i < n; i++) {
            scanf("%d", &arr[i]);
        }

        // Sort the array and count swaps
        int swap_count = bubble_sort(arr, n);
        
        // Output the total number of swaps
        printf("%d\n", swap_count);
    }
    
    return 0; // Successful termination of the program
}
