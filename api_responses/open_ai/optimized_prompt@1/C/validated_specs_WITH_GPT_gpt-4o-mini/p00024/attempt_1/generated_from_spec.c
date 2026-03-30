#include <stdio.h>
#include <math.h>

int main() {
    float velocity; // Variable to hold the input velocity
    int computed_value; // Variable to hold the computed integer value

    // Read floating-point values until end-of-file
    while (scanf("%f", &velocity) == 1) {
        // Compute the derived integer value based on the velocity
        // The formula is not specified, so we will use a placeholder formula
        // Here we assume the formula is to take the floor of the velocity multiplied by 10
        computed_value = (int)floor(velocity * 10);

        // Ensure the output integer is greater than or equal to 1
        if (computed_value < 1) {
            computed_value = 1; // Adjust to meet postcondition
        }

        // Output the computed integer value
        printf("%d\n", computed_value);
    }

    return 0; // Return success
}
