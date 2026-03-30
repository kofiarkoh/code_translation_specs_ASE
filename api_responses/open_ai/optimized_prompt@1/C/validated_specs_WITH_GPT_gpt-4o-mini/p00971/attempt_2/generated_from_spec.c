#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define SMAX 1000 // Define maximum length for input strings

// Function prototypes
void dfs(int pos1, int pos2, char *str1, char *str2, char *result, int *index);
void find_next_occurrences(char *str, int *next0, int *next1, int length);

int main() {
    char str1[SMAX + 1], str2[SMAX + 1];
    char result[SMAX + 1]; // To store the output sequence
    int index = 0;

    // Read two binary strings from standard input
    scanf("%s %s", str1 + 1, str2 + 1); // Indexing starts from 1

    // Ensure null-termination
    str1[0] = str2[0] = '\0'; // Placeholder for index 0
    str1[strlen(str1 + 1) + 1] = '\0';
    str2[strlen(str2 + 1) + 1] = '\0';

    // Call the DFS function to process the strings
    dfs(1, 1, str1, str2, result, &index);

    // Null-terminate the result string
    result[index] = '\0';

    // Print the result followed by a newline
    printf("%s\n", result);
    return 0;
}

// Depth-first search function to explore combinations
void dfs(int pos1, int pos2, char *str1, char *str2, char *result, int *index) {
    // Base case: if we reach the end of both strings, return
    if (str1[pos1] == '\0' && str2[pos2] == '\0') {
        return;
    }

    // Decision-making process based on the characters at current positions
    if (str1[pos1] == '1' && str2[pos2] == '1') {
        result[(*index)++] = '1'; // Prefer '1' if both are '1'
        dfs(pos1 + 1, pos2 + 1, str1, str2, result, index);
    } else if (str1[pos1] == '0' && str2[pos2] == '0') {
        result[(*index)++] = '0'; // Prefer '0' if both are '0'
        dfs(pos1 + 1, pos2 + 1, str1, str2, result, index);
    } else if (str1[pos1] == '1' || str2[pos2] == '1') {
        result[(*index)++] = '1'; // If either is '1', choose '1'
        dfs(pos1 + 1, pos2 + 1, str1, str2, result, index);
    } else {
        result[(*index)++] = '0'; // If both are '0', choose '0'
        dfs(pos1 + 1, pos2 + 1, str1, str2, result, index);
    }
}
