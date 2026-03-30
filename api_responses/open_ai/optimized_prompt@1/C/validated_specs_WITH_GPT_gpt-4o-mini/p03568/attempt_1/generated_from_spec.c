#include <stdio.h>
#include <math.h>

int compute_value(int N, int *values) {
    // Check for the precondition that N is non-negative
    if (N < 0) {
        // Undefined behavior as per specification
        return 0; // Placeholder for undefined behavior
    }

    int count_even = 0;

    // Count the number of even integers
    for (int i = 0; i < N; i++) {
        if (values[i] % 2 == 0) {
            count_even++;
        }
    }

    // Calculate 3^N and 2^count_even
    // Using pow function to compute powers
    // Note: pow returns a double, we cast it to int for the final result
    int power_of_three = (int)pow(3, N);
    int power_of_two = (int)pow(2, count_even);

    // Return the result of the expression 3^N - 2^count_even
    return power_of_three - power_of_two;
}

int main() {
    int N;
    // Read the number of integers
    scanf("%d", &N);

    // Declare an array to hold the integers
    int values[N];

    // Read the N integers
    for (int i = 0; i < N; i++) {
        scanf("%d", &values[i]);
    }

    // Compute the result
    int result = compute_value(N, values);

    // Output the result
    printf("%d\n", result);

    return 0;
}
