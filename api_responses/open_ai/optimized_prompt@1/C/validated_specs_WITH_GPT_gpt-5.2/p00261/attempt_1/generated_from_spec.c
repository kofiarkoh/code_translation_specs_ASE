#include <stdio.h>

int main(void) {
    /* Fixed internal buffer: at most 109 characters excluding terminating null */
    char tok[110];

    /* State encoding: S0..S5 -> 0..5. Accepting state is S5 (5). */
    enum { S0 = 0, S1, S2, S3, S4, S5 };

    /* Transition table for inputs '0' and '1'.
       -1 denotes undefined transition (stop processing current token). */
    static const int trans[6][2] = {
        /* S0 */ { S1, S2 },
        /* S1 */ { -1, S3 },
        /* S2 */ { S1, -1 },
        /* S3 */ { S4, S5 },
        /* S4 */ { S5, S2 },
        /* S5 */ { S2, S1 }
    };

    while (scanf("%109s", tok) == 1) {
        if (tok[0] == '#')
            break;

        int state = S0;

        for (int i = 0; tok[i] != '\0'; i++) {
            /* Preconditions say only '0'/'1' appear; otherwise behavior is undefined.
               We intentionally do not validate beyond mapping to 0/1. */
            int sym = tok[i] - '0'; /* '0'->0, '1'->1 */
            int next = trans[state][sym];
            if (next < 0) {
                /* Undefined transition: stop immediately, keep current state. */
                break;
            }
            state = next;
        }

        if (state == S5) {
            puts("YES");
        } else {
            puts("NO");
        }
    }

    return 0;
}
