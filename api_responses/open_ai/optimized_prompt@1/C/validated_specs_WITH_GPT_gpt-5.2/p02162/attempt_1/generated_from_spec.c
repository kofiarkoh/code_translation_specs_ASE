#include <stdio.h>

int main(void) {
    int cmp1, cmp2;
    int res1, res2;

    if (scanf("%d %d %d %d", &cmp1, &cmp2, &res1, &res2) != 4) {
        /* Fewer than four integers: produce no output and exit non-zero. */
        return 1;
    }

    const char *out;

    if (res1 < 0 || res2 < 0) {
        /* At least one result is negative: compare comparison values. */
        if (cmp1 > cmp2) out = "First";
        else if (cmp2 > cmp1) out = "Second";
        else out = "Draw";
    } else {
        /* Both results are non-negative: compare result values. */
        if (res1 > res2) out = "First";
        else if (res2 > res1) out = "Second";
        else out = "Draw";
    }

    printf("%s\n", out);
    return 0;
}
