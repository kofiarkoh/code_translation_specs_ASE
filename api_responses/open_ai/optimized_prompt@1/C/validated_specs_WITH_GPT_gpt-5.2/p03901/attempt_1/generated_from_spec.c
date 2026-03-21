#include <stdio.h>

static double g_p = 0.0;

static double expected_value(int position) {
    if ((position % 2) == 0) {
        /* Even position base rule: (position/2) / probability.
           Use integer division for position/2 as specified by "half-position". */
        return ((double)(position / 2)) / g_p;
    } else {
        /* Odd position recurrence:
           1 + value(position-1)*p + value(position+1)*(1-p) */
        return 1.0
             + expected_value(position - 1) * g_p
             + expected_value(position + 1) * (1.0 - g_p);
    }
}

int main(void) {
    int position;
    double percent;

    /* No explicit validation per specification. */
    if (scanf("%d", &position) != 1) return 0;
    if (scanf("%lf", &percent) != 1) return 0;

    g_p = percent / 100.0;

    double result = expected_value(position);
    printf("%.7f\n", result);

    return 0;
}
