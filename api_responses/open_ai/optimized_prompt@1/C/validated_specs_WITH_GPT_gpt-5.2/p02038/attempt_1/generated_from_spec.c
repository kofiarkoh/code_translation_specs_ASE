#include <stdio.h>
#include <stdlib.h>

int main(void) {
    /* Fixed-size internal storage capacity for tokens (first character only). */
    enum { MAX_TOKENS = 100000 };
    char tokens[MAX_TOKENS];

    int n;
    if (scanf("%d", &n) != 1) {
        /* No guaranteed output on failure. */
        return 1;
    }

    /* Behavior is undefined by the specification if n < 2 or n > MAX_TOKENS.
       We intentionally do not add explicit handling beyond what naturally occurs. */

    for (int i = 0; i < n; i++) {
        char buf[1024];
        if (scanf("%1023s", buf) != 1) {
            /* No guaranteed output on failure. */
            return 1;
        }
        tokens[i] = buf[0];
    }

    /* Reduction rule:
       result is false only when left operand is true and right operand is false;
       otherwise result is true.
       Only 'T' is treated as true, only 'F' is treated as false; other chars are
       simply "not 'T'" / "not 'F'" as per the rule. */
    char acc = tokens[0];
    for (int i = 1; i < n; i++) {
        char left = acc;
        char right = tokens[i];
        if (left == 'T' && right == 'F') {
            acc = 'F';
        } else {
            acc = 'T';
        }
    }

    /* Invariant: intermediate result is always 'T' or 'F' after first reduction step.
       Output final reduced truth value followed by a line terminator. */
    putchar(acc);
    putchar('\n');

    return 0;
}
