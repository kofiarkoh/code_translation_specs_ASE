#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRING_LENGTH 79
#define MAX_UNIQUE_VARIATIONS 1000 // Arbitrary limit for unique variations

// Function to reverse a substring in place
void reverse(char *str, int start, int end) {
    while (start < end) {
        char temp = str[start];
        str[start] = str[end];
        str[end] = temp;
        start++;
        end--;
    }
}

// Function to swap two segments of the string
void swap(char *str, int index) {
    int len = strlen(str);
    if (index < len) {
        char *temp = (char *)malloc(len + 1);
        strcpy(temp, str);
        for (int i = 0; i < index; i++) {
            str[i] = temp[index + i];
        }
        for (int i = index; i < len; i++) {
            str[i] = temp[i - index];
        }
        free(temp);
    }
}

// Function to check if a string is unique in the array
int is_unique(char **unique_variations, int count, const char *variation) {
    for (int i = 0; i < count; i++) {
        if (strcmp(unique_variations[i], variation) == 0) {
            return 0; // Not unique
        }
    }
    return 1; // Unique
}

// Main function to process strings and generate unique variations
int process_strings(int num_strings, char strings[][MAX_STRING_LENGTH + 1]) {
    for (int s = 0; s < num_strings; s++) {
        char original[MAX_STRING_LENGTH + 1];
        strcpy(original, strings[s]);
        char *unique_variations[MAX_UNIQUE_VARIATIONS];
        int unique_count = 0;

        // Store the original string if unique
        if (is_unique(unique_variations, unique_count, original)) {
            unique_variations[unique_count++] = strdup(original);
        }

        int len = strlen(original);
        for (int i = 1; i < len; i++) {
            // Reverse first i characters
            reverse(original, 0, i - 1);
            if (is_unique(unique_variations, unique_count, original)) {
                unique_variations[unique_count++] = strdup(original);
            }
            // Reverse last len-i characters
            reverse(original, i, len - 1);
            if (is_unique(unique_variations, unique_count, original)) {
                unique_variations[unique_count++] = strdup(original);
            }
            // Restore original and swap
            reverse(original, 0, len - 1);
            swap(original, i);
            if (is_unique(unique_variations, unique_count, original)) {
                unique_variations[unique_count++] = strdup(original);
            }
            // Restore original for next iteration
            reverse(original, 0, len - 1);
        }

        // Output the count of unique variations
        printf("%d\n", unique_count);

        // Free allocated memory for unique variations
        for (int j = 0; j < unique_count; j++) {
            free(unique_variations[j]);
        }
    }
    return 0;
}

int main() {
    int num_strings;
    scanf("%d", &num_strings);
    if (num_strings < 0) {
        return 1; // Invalid input
    }

    char strings[num_strings][MAX_STRING_LENGTH + 1];
    for (int i = 0; i < num_strings; i++) {
        scanf("%s", strings[i]);
        // Behavior is undefined if the string exceeds the maximum length
    }

    process_strings(num_strings, strings);
    return 0;
}
