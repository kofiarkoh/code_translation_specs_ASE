#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 1000 // Assuming a maximum size for the arrays

// Function to compare two integers for descending order sorting
int compare(const void *a, const void *b) {
    return (*(int *)b - *(int *)a);
}

int main() {
    int X, Y, A, B, C;
    int firstArray[MAX_SIZE], secondArray[MAX_SIZE], thirdArray[MAX_SIZE];
    int sum = 0;

    // Read the five integers
    if (scanf("%d %d %d %d %d", &X, &Y, &A, &B, &C) != 5) {
        return 1; // Error code for input mismatch
    }

    // Read the first array
    for (int i = 0; i < A; i++) {
        if (scanf("%d", &firstArray[i]) != 1) {
            return 1; // Error code for input reading failure
        }
    }

    // Read the second array
    for (int i = 0; i < B; i++) {
        if (scanf("%d", &secondArray[i]) != 1) {
            return 1; // Error code for input reading failure
        }
    }

    // Read the third array
    for (int i = 0; i < C; i++) {
        if (scanf("%d", &thirdArray[i]) != 1) {
            return 1; // Error code for input reading failure
        }
    }

    // Sort the first and second arrays in descending order
    qsort(firstArray, A, sizeof(int), compare);
    qsort(secondArray, B, sizeof(int), compare);

    // Process the first array with the third array
    for (int i = 0; i < A && i < C; i++) {
        if (thirdArray[i] > firstArray[i]) {
            firstArray[i] = thirdArray[i]; // Replace if third array element is greater
        }
    }

    // Process the second array with the third array
    for (int i = 0; i < B && i < C; i++) {
        if (thirdArray[i] > secondArray[i]) {
            secondArray[i] = thirdArray[i]; // Replace if third array element is greater
        }
    }

    // Calculate the sum of the first X elements from the first array
    for (int i = 0; i < X && i < A; i++) {
        sum += firstArray[i];
    }

    // Calculate the sum of the first Y elements from the second array
    for (int i = 0; i < Y && i < B; i++) {
        sum += secondArray[i];
    }

    // Output the result
    printf("%d\n", sum);
    return 0; // Successful execution
}
