#include <stdio.h>

int main(void) {
    int n, m;
    int a[1000];

    if (scanf("%d %d", &n, &m) != 2) {
        return 0; /* unspecified behavior per spec; exit quietly if input missing */
    }

    for (int i = 0; i < n; i++) {
        if (scanf("%d", &a[i]) != 1) {
            return 0; /* unspecified behavior per spec; exit quietly if input missing */
        }
    }

    for (int k = 1; k <= m; k++) {
        for (int i = 0; i < n - 1; i++) {
            /* Use C's % operator directly to match language remainder semantics,
               including behavior for negative values. */
            int left_rem = a[i] % k;
            int right_rem = a[i + 1] % k;
            if (left_rem > right_rem) {
                int tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;
            }
        }
    }

    for (int i = 0; i < n; i++) {
        printf("%d\n", a[i]);
    }

    return 0;
}
