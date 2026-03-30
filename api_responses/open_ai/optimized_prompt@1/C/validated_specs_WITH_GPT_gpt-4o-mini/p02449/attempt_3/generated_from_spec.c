#include <stdio.h>

void print_array(int arr[], int n) {
    for (int i = 0; i < n; i++) {
        printf("%d", arr[i]);
        if (i < n - 1) {
            printf(" ");
        }
    }
    printf("\n");
}

int previous_permutation(int arr[], int n) {
    int i = n - 2;
    // Find the rightmost pair where arr[i] > arr[i + 1]
    while (i >= 0 && arr[i] <= arr[i + 1]) {
        i--;
    }
    if (i < 0) {
        return 0; // No previous permutation
    }
    
    int j = n - 1;
    // Find the largest element to the right of arr[i] that is smaller than arr[i]
    while (arr[j] >= arr[i]) {
        j--;
    }
    // Swap arr[i] and arr[j]
    int temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
    
    // Reverse the sequence from arr[i + 1] to the end
    for (int k = i + 1, l = n - 1; k < l; k++, l--) {
        temp = arr[k];
        arr[k] = arr[l];
        arr[l] = temp;
    }
    return 1; // Previous permutation exists
}

int next_permutation(int arr[], int n) {
    int i = n - 2;
    // Find the rightmost pair where arr[i] < arr[i + 1]
    while (i >= 0 && arr[i] >= arr[i + 1]) {
        i--;
    }
    if (i < 0) {
        return 0; // No next permutation
    }
    
    int j = n - 1;
    // Find the smallest element to the right of arr[i] that is larger than arr[i]
    while (arr[j] <= arr[i]) {
        j--;
    }
    // Swap arr[i] and arr[j]
    int temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
    
    // Reverse the sequence from arr[i + 1] to the end
    for (int k = i + 1, l = n - 1; k < l; k++, l--) {
        temp = arr[k];
        arr[k] = arr[l];
        arr[l] = temp;
    }
    return 1; // Next permutation exists
}

int main() {
    int n;
    scanf("%d", &n); // Read the number of elements

    // Ensure n is within the specified range
    if (n < 1 || n > 9) {
        return 1; // Undefined behavior as per specification
    }

    int arr[9]; // Array to hold the sequence
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]); // Read the sequence
    }

    int prev_arr[9], next_arr[9];
    for (int i = 0; i < n; i++) {
        prev_arr[i] = arr[i]; // Copy original for previous permutation
        next_arr[i] = arr[i]; // Copy original for next permutation
    }

    // Compute previous permutation
    if (previous_permutation(prev_arr, n)) {
        print_array(prev_arr, n); // Print previous permutation
    }

    // Print original sequence
    print_array(arr, n);

    // Compute next permutation
    if (next_permutation(next_arr, n)) {
        print_array(next_arr, n); // Print next permutation
    }

    return 0;
}
