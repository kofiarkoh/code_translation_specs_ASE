#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_WORDS 50000
#define MAX_LENGTH 204

// Function to check if a word matches a query with wildcards
int matches(const char *word, const char *query) {
    const char *w = word;
    const char *q = query;
    const char *star = NULL;
    const char *match = NULL;

    while (*w) {
        if (*q == '?' || *q == *w) {
            w++;
            q++;
        } else if (*q == '*') {
            star = q++;
            match = w;
        } else if (star) {
            q = star + 1;
            w = ++match;
        } else {
            return 0;
        }
    }

    while (*q == '*') {
        q++;
    }

    return !*q;
}

int main() {
    int N, M;
    scanf("%d %d", &N, &M);
    
    // Check for valid input
    if (N < 0 || M < 0 || N > MAX_WORDS) {
        return 1; // Undefined behavior for invalid input
    }

    char words[MAX_WORDS][MAX_LENGTH + 1];
    char reversed[MAX_WORDS][MAX_LENGTH + 1];

    // Read words and their reversed forms
    for (int i = 0; i < N; i++) {
        scanf("%s", words[i]);
        // Check word length
        if (strlen(words[i]) > MAX_LENGTH) {
            return 1; // Undefined behavior for exceeding max length
        }
        // Create reversed word
        int len = strlen(words[i]);
        for (int j = 0; j < len; j++) {
            reversed[i][j] = words[i][len - 1 - j];
        }
        reversed[i][len] = '\0';
    }

    // Process queries
    for (int i = 0; i < M; i++) {
        char query[MAX_LENGTH + 1];
        scanf("%s", query);

        // Check for empty or whitespace-only query
        if (strlen(query) == 0 || strspn(query, " ") == strlen(query)) {
            printf("0\n");
            continue;
        }

        int count = 0;

        // Count matches in words and reversed words
        for (int j = 0; j < N; j++) {
            if (matches(words[j], query) || matches(reversed[j], query)) {
                count++;
            }
        }

        printf("%d\n", count);
    }

    return 0;
}
