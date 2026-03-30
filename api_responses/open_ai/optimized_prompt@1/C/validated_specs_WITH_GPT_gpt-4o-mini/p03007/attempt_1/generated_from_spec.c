#include <stdio.h>
#include <stdlib.h>

void process_array(int n, long long *arr) {
    // Step 1: Sort the array in non-decreasing order
    for (int i = 0; i < n - 1; i++) {
        for (int j = 0; j < n - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                long long temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }

    // Step 2: Compute the sum of absolute values
    long long sum = 0;
    for (int i = 0; i < n; i++) {
        sum += (arr[i] < 0) ? -arr[i] : arr[i]; // Add absolute value
    }

    // Step 3: Print the computed sum
    printf("%lld\n", sum);

    // Step 4: Print pairs of values from the array
    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            printf("(%lld, %lld)\n", arr[i], arr[j]); // Print pairs
        }
    }
}

int main() {
    int n;
    // Read the number of elements
    scanf("%d", &n);
    
    // Allocate memory for the array
    long long *arr = (long long *)malloc(n * sizeof(long long));
    
    // Read the array elements
    for (int i = 0; i < n; i++) {
        scanf("%lld", &arr[i]);
    }

    // Process the array
    process_array(n, arr);

    // Free allocated memory
    free(arr);
    return 0;
}
