#include <stdio.h>

int main(void) {
    int h, w;
    if (scanf("%d %d", &h, &w) != 2) {
        return 0; /* No explicit malformed-input handling per spec */
    }

    /* Fixed-size storage per stated maximums (0..51, 0..59). */
    char s[51][60];

    for (int i = 0; i < h; i++) {
        /* No explicit handling for short rows; reads a token (no spaces). */
        if (scanf("%59s", s[i]) != 1) {
            return 0;
        }
    }

    for (int i = 0; i < h; i++) {
        for (int j = 0; j < w; j++) {
            if (s[i][j] == '#') {
                int ok = 0;

                /* Orthogonal neighbors only. */
                if (i > 0 && s[i - 1][j] == '#') ok = 1;
                if (i + 1 < h && s[i + 1][j] == '#') ok = 1;
                if (j > 0 && s[i][j - 1] == '#') ok = 1;
                if (j + 1 < w && s[i][j + 1] == '#') ok = 1;

                if (!ok) {
                    /* Terminate immediately upon detecting a violating marked cell. */
                    puts("No");
                    return 0;
                }
            }
        }
    }

    puts("Yes");
    return 0;
}
