#include <stdio.h>

int main() {
    int num_cases; // Number of cases to process
    scanf("%d", &num_cases); // Read the number of cases

    // Preconditions: num_cases must be a positive integer
    for (int case_num = 1; case_num <= num_cases; case_num++) {
        int input_value; // Input integer for calculations
        scanf("%d", &input_value); // Read the input integer

        // Output the case header
        printf("Case %d:\n", case_num);

        // Compute and output ten derived values
        for (int i = 0; i < 10; i++) {
            // Perform the specified calculations
            int derived_value = (input_value * input_value / 100) % 10000;
            printf("%d\n", derived_value);
        }
    }

    return 0; // End of program
}
