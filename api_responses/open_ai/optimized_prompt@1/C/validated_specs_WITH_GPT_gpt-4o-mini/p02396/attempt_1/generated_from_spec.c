#include <stdio.h>

int main() {
    int input; // Variable to store user input
    int case_number = 1; // Initialize case number

    // Continuous loop to read inputs
    while (1) {
        // Read an integer from the user
        scanf("%d", &input);

        // Check for termination condition
        if (input == 0) {
            break; // Exit the loop if input is zero
        }

        // Output the case number and the integer value
        printf("Case %d: %d\n", case_number, input);
        
        // Increment the case number for the next valid input
        case_number++;
    }

    return 0; // Terminate the program
}
