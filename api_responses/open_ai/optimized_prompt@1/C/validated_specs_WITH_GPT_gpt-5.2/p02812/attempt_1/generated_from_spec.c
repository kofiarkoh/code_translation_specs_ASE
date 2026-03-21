#include <stdio.h>

int main(void) {
    int n;
    char s[51]; /* token length does not exceed 50 characters */

    if (scanf("%d %50s", &n, s) != 2) {
        /* Unspecified behavior per spec if input is missing/malformed;
           return without producing defined output. */
        return 0;
    }

    int count = 0;

    if (n >= 3) {
        int i = 0;
        while (i <= n - 3) {
            if (s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C') {
                count++;
                i += 3; /* enforce non-overlapping occurrences */
            } else {
                i += 1;
            }
        }
    }

    printf("%d\n", count);
    return 0;
}
