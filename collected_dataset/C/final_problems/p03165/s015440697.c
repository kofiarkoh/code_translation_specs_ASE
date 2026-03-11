#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

struct Node {
    struct Node * previous;
    int matched;
    char value;
    bool selected;
} typedef tNode;

void printNode(tNode * node) {
    if (node->matched > 0) {
        if (node->previous != NULL) {
            printNode(node->previous);
        }

        if (node->selected) {
            printf("%c", node->value);
        }
    } else {
    }
}

void printLargestSubstring(char * s, char * t, int n, int m) {
    int i, j;
    tNode matches[n][m];

    for (i = 0; i < n; i++) {
        for (j = 0; j < m; j++) {
            matches[i][j].matched = 0;
            matches[i][j].previous = NULL;
            matches[i][j].value = '\0';
            matches[i][j].selected = false;
        }
    }

    for (i = 0; i < n; i++) {
        for (j = 0; j < m; j++) {
            matches[i][j].value = s[i];

            if (matches[i][j].value == t[j]) {
                matches[i][j].matched = 1;
                matches[i][j].selected = true;
            } else {
                matches[i][j].matched = 0;
                matches[i][j].selected = false;
            }

            if (i > 0 && j > 0) {
                matches[i][j].previous = &matches[i - 1][j - 1];
                matches[i][j].matched += matches[i][j].previous->matched;
            }

            if (i > 0) {
                if (matches[i - 1][j].matched > matches[i][j].matched) {
                    matches[i][j].previous = &matches[i - 1][j];
                    matches[i][j].matched = matches[i - 1][j].matched;
                    matches[i][j].selected = false;
                }
            }

            if (j > 0) {
                if (matches[i][j - 1].matched > matches[i][j].matched) {
                    matches[i][j].previous = &matches[i][j - 1];
                    matches[i][j].matched = matches[i][j - 1].matched;
                    matches[i][j].selected = false;
                }
            }
        }
    }

    printNode(&matches[n - 1][m - 1]);

    printf("\n");
}

int main() {
    char s[3000], t[3000];

    fgets(s, sizeof(s), stdin);
    fgets(t, sizeof(t), stdin);

    int n = strlen(s) - 1;
    int m = strlen(t) - 1;

    if (n < m) {
        printLargestSubstring(s, t, n, m);
    } else {
        printLargestSubstring(t, s, m, n);
    }

	return 0;
}
