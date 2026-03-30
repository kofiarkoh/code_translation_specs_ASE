#include <stdio.h>
#include <math.h>

int main() {
    int input; // Variable to store the input integer

    // Loop to read input until termination condition is met
    while (1) {
        // Read an integer from standard input
        if (scanf("%d", &input) != 1) {
            // If input is not an integer, behavior is undefined as per specification
            break; // Exit loop on invalid input
        }

        // Check for termination condition
        if (input == -1) {
            break; // Exit loop if termination value is encountered
        }

        // Ensure input is non-negative as per preconditions
        if (input < 0) {
            continue; // Skip negative inputs (though preconditions state valid inputs)
        }

        // Compute the cube root using the pow function
        double cube_root = pow((double)input, 1.0 / 3.0);

        // Output the cube root formatted to six decimal places
        printf("%.6f\n", cube_root);
    }

    return 0; // Successful termination of the program
}
