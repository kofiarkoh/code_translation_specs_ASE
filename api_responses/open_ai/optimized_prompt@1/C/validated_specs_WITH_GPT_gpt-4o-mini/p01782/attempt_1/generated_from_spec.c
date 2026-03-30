#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LINES 100
#define MAX_CHARS 52

// Function to decode the messages based on character mappings
void decode_messages(int N, char messages[MAX_LINES][MAX_CHARS], char *decoded) {
    int freq[52] = {0}; // Frequency array for 26 uppercase + 26 lowercase letters
    int index;

    // Count frequency of each character in the input messages
    for (int i = 0; i < N; i++) {
        for (int j = 0; messages[i][j] != '\0'; j++) {
            if (messages[i][j] >= 'A' && messages[i][j] <= 'Z') {
                index = messages[i][j] - 'A'; // Uppercase letters
            } else if (messages[i][j] >= 'a' && messages[i][j] <= 'z') {
                index = messages[i][j] - 'a' + 26; // Lowercase letters
            } else {
                continue; // Ignore invalid characters (not specified to handle)
            }
            freq[index]++;
        }
    }

    // Construct the decoded string based on frequency
    int pos = 0;
    for (int i = 0; i < 52; i++) {
        while (freq[i] > 0) {
            if (i < 26) {
                decoded[pos++] = 'A' + i; // Uppercase letters
            } else {
                decoded[pos++] = 'a' + (i - 26); // Lowercase letters
            }
            freq[i]--;
        }
    }
    decoded[pos] = '\0'; // Null-terminate the decoded string
}

int main() {
    int N;
    char messages[MAX_LINES][MAX_CHARS];
    char decoded[MAX_LINES * MAX_CHARS]; // Buffer for decoded output

    // Read the number of lines
    if (scanf("%d", &N) != 1 || N < 0 || N > MAX_LINES) {
        return 1; // Invalid input handling (not specified, but safe to exit)
    }

    // Read the messages
    for (int i = 0; i < N; i++) {
        scanf("%s", messages[i]);
    }

    // Decode the messages
    decode_messages(N, messages, decoded);

    // Output the decoded string
    printf("%s\n", decoded);

    return 0;
}
