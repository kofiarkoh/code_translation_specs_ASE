#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LENGTH 80
#define MAX_SUM_LIMIT 999999999999999999 // Define a limit for the sum

// Function to convert a string representing a non-negative integer to an array of digits
void stringToDigits(const char *str, int *digits, int *length) {
    *length = strlen(str);
    for (int i = 0; i < *length; i++) {
        digits[i] = str[*length - 1 - i] - '0'; // Store digits in reverse order
    }
}

// Function to compute the sum of two arrays of digits
int computeSum(int *digits1, int len1, int *digits2, int len2, int *result) {
    int carry = 0;
    int maxLength = len1 > len2 ? len1 : len2;
    int sumLength = 0;

    for (int i = 0; i < maxLength || carry; i++) {
        int digit1 = (i < len1) ? digits1[i] : 0;
        int digit2 = (i < len2) ? digits2[i] : 0;
        int sum = digit1 + digit2 + carry;

        if (sum > 9) {
            carry = 1;
            sum -= 10;
        } else {
            carry = 0;
        }

        result[sumLength++] = sum; // Store the result digit
    }

    return sumLength; // Return the length of the result
}

// Function to print the result as a string
void printResult(int *result, int length) {
    for (int i = length - 1; i >= 0; i--) {
        printf("%d", result[i]);
    }
    printf("\n");
}

int main() {
    int numDatasets;
    scanf("%d", &numDatasets); // Read the number of datasets

    for (int i = 0; i < numDatasets; i++) {
        char str1[MAX_LENGTH + 1], str2[MAX_LENGTH + 1];
        int digits1[MAX_LENGTH] = {0}, digits2[MAX_LENGTH] = {0};
        int result[MAX_LENGTH + 1] = {0}; // Extra space for potential carry
        int len1, len2;

        // Read the two strings
        scanf("%s %s", str1, str2);

        // Check for overflow in string length
        if (strlen(str1) > MAX_LENGTH || strlen(str2) > MAX_LENGTH) {
            printf("Overflow\n");
            continue; // Skip to the next dataset
        }

        // Convert strings to digit arrays
        stringToDigits(str1, digits1, &len1);
        stringToDigits(str2, digits2, &len2);

        // Compute the sum
        int sumLength = computeSum(digits1, len1, digits2, len2, result);

        // Check for overflow in computed sum
        if (sumLength > 18 || (sumLength == 18 && result[sumLength - 1] > 9)) {
            printf("Overflow\n");
        } else {
            printResult(result, sumLength); // Print the result
        }
    }

    return 0;
}
