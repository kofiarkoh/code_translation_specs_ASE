#include <stdio.h>

#define MAXN 1000000

static int a[MAXN];

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        return 0; /* Undefined behavior per spec; exit quietly on missing input */
    }

    /* Spec: fixed global storage capacity up to 1,000,000 integers */
    for (int i = 0; i < n; i++) {
        (void)scanf("%d", &a[i]); /* No explicit error handling per spec */
    }

    /* Single partitioning step around last element as pivot (Lomuto partition). */
    int pivot = a[n - 1];
    int i = -1;
    for (int j = 0; j < n - 1; j++) {
        if (a[j] <= pivot) {
            i++;
            int tmp = a[i];
            a[i] = a[j];
            a[j] = tmp;
        }
    }
    int pivot_index = i + 1;
    {
        int tmp = a[pivot_index];
        a[pivot_index] = a[n - 1];
        a[n - 1] = tmp;
    }

    /* Output: single line, space-separated; exactly one element (pivot final position) bracketed. */
    for (int idx = 0; idx < n; idx++) {
        if (idx > 0) putchar(' ');
        if (idx == pivot_index) {
            printf("[%d]", a[idx]);
        } else {
            printf("%d", a[idx]);
        }
    }
    putchar('\n');

    return 0;
}
