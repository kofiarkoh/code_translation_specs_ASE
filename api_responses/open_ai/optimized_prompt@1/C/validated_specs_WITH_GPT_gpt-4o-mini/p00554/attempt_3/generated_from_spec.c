#include <stdio.h>

#define MAX_PAIRS 100 // Assuming a maximum size for the arrays

int main() {
    int n, m;
    int firstArray[MAX_PAIRS], secondArray[MAX_PAIRS];
    int count1 = 0; // Count of pairs where first integer >= n

    // Read inputs
    scanf("%d %d", &n, &m);
    
    // Preconditions check (not explicitly handled, as per specification)
    for (int i = 0; i < m; i++) {
        int a, b;
        scanf("%d %d", &a, &b);
        firstArray[i] = a;
        secondArray[i] = b;

        // Modify arrays based on the threshold condition
        if (a >= n) {
            firstArray[i] = 0; // Store original first integers
            secondArray[i] = 0; // Set second array value to 0
            count1++;
        }
    }

    // If count1 >= m - 1, output will be 0
    if (count1 >= m - 1) {
        printf("0\n");
        return 0;
    }

    // Sort the second array using a simple bubble sort (O(m^2) complexity)
    for (int i = 0; i < m - count1 - 1; i++) {
        for (int j = 0; j < m - count1 - 1 - i; j++) {
            if (secondArray[j] > secondArray[j + 1]) {
                // Swap values
                int temp = secondArray[j];
                secondArray[j] = secondArray[j + 1];
                secondArray[j + 1] = temp;
            }
        }
    }

    // Calculate the cumulative sum of the first m - count1 - 1 elements
    int cumulativeSum = 0;
    for (int i = 0; i < m - count1 - 1; i++) {
        cumulativeSum += secondArray[i];
    }

    // Output the cumulative sum
    printf("%d\n", cumulativeSum);
    return 0;
}
