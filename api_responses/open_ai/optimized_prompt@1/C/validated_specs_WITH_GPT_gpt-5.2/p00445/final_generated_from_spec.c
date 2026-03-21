#include <stdio.h>
#include <string.h>

int main(void) {
    /* Fixed maximum token size of 10000 characters (plus NUL). */
    char s[10000 + 1];

    /* Read zero or more whitespace-delimited tokens until EOF. */
    while (scanf("%10000s", s) == 1) {
        int joi = 0;
        int ioi = 0;

        /* Repeated length evaluation as implied by the specification. */
        for (size_t i = 0; i + 2 < strlen(s); i++) {
            if (s[i] == 'J' && s[i + 1] == 'O' && s[i + 2] == 'I') {
                joi++;
            }
            if (s[i] == 'I' && s[i + 1] == 'O' && s[i + 2] == 'I') {
                ioi++;
            }
        }

        printf("%d\n%d\n", joi, ioi);
    }

    return 0;
}
