#include <stdio.h>

int main(void) {
    char token[100]; /* fixed internal buffer capacity: 99 chars + terminator */

    while (scanf("%99s", token) == 1) {
        for (int i = 0; token[i] != '\0'; i++) {
            if (token[i] == '@') { /* repetition marker */
                int d = token[i + 1] - '0'; /* precondition: token[i+1] is a digit */
                char c = token[i + 2];      /* precondition: token[i+2] exists */
                for (int k = 0; k < d; k++) {
                    putchar((unsigned char)c);
                }
                i += 2; /* consume '@', digit, and the repeated character */
            } else {
                putchar((unsigned char)token[i]);
            }
        }
        putchar('\n');
    }

    return 0;
}
