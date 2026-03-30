#include <stdio.h>
#include <string.h>

#define MAX_LENGTH 10000
#define SUBSTR1 "JOI"
#define SUBSTR2 "IOI"

int count_occurrences(const char *str, const char *substr) {
    int count = 0;
    const char *ptr = str;

    // Count occurrences of the substring in the string
    while ((ptr = strstr(ptr, substr)) != NULL) {
        count++;
        ptr++; // Move past the current found substring to continue searching
    }
    return count;
}

int main() {
    char input[MAX_LENGTH + 1]; // +1 for null terminator

    // Read input until EOF
    while (fgets(input, sizeof(input), stdin) != NULL) {
        // Remove newline character if present
        input[strcspn(input, "\n")] = 0;

        // Count occurrences of "JOI" and "IOI"
        int joi_count = count_occurrences(input, SUBSTR1);
        int ioi_count = count_occurrences(input, SUBSTR2);

        // Output the counts
        printf("%d %d\n", joi_count, ioi_count);
    }

    return 0;
}
