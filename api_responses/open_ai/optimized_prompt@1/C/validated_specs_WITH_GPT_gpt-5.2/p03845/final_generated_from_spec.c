#include <stdio.h>

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    int a[100];
    int S = 0;

    for (int i = 0; i < n; i++) {
        scanf("%d", &a[i]);
        S += a[i];
    }

    int m;
    scanf("%d", &m);

    for (int i = 0; i < m; i++) {
        int p, x;
        scanf("%d %d", &p, &x);
        /* p is 1-based per specification; behavior undefined if out of range */
        int result = S - a[p - 1] + x;
        printf("%d\n", result);
    }

    return 0;
}
