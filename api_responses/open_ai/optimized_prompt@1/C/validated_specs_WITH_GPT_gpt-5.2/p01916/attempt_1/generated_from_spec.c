#include <stdio.h>
#include <string.h>

int main(void) {
    char s[1010];
    if (scanf("%1009s", s) != 1) {
        /* Undefined behavior per specification if input is missing;
           we simply exit without producing output. */
        return 0;
    }

    /* Track parity (even/odd) for each letter. */
    unsigned char parity[26] = {0};

    for (size_t i = 0; s[i] != '\0'; ++i) {
        /* Preconditions guarantee 'a'..'z'; otherwise behavior is undefined. */
        parity[(unsigned)(s[i] - 'a')] ^= 1u;
    }

    int odd_count = 0;
    for (int i = 0; i < 26; ++i) {
        odd_count += (parity[i] & 1u);
    }

    /* Output floor(odd_count / 2). */
    printf("%d", odd_count / 2);
    return 0;
}
