#include <stdio.h>

int main(void) {
    char s[200]; /* fixed internal storage: up to 199 chars + '\0' */

    /* No explicit error handling per spec; if input cannot be read, behavior is undefined. */
    (void)fgets(s, sizeof(s), stdin);

    /* Fixed-size run counts indexed by character value.
       Assumption: input characters map into a non-negative range suitable for counting. */
    int runs[256] = {0};

    /* Scan left-to-right, counting runs: increment when current char differs from previous. */
    unsigned char prev = 0;
    int have_prev = 0;

    for (int i = 0; s[i] != '\0' && s[i] != '\n'; i++) {
        unsigned char cur = (unsigned char)s[i];
        if (!have_prev || cur != prev) {
            runs[cur]++;
            prev = cur;
            have_prev = 1;
        }
    }

    putchar((runs[(unsigned char)'o'] >= runs[(unsigned char)'x']) ? 'o' : 'x');
    putchar('\n');
    return 0;
}
