/*
 * ID: 3001
 * Proble: GPA janken, combination
 */

#include <stdio.h>

int main(void)
{
    int a[500000];
    int c[4001] = {};
    int s[4001] = {};

    int n;
    scanf("%d", &n);

    int i;
    for (i = 0; i < n; i++) {
        int d, e, f, g;
        scanf("%1d %*1c %1d %1d %1d", &d, &e, &f, &g);
        a[i] = ((d * 10 + e) * 10 + f) * 10 + g;
    }

    for (i = 0; i < n; i++) 
        c[a[i]]++;

    int k = n - 1;
    for (i = 4000; i >= 0 && k > 0; i--) {
        if (c[i] == 0)
            continue;
        s[i] = k * 3 - (c[i] - 1) * 2;
        k -= c[i];
    }

    for (i = 0; i < n; i++)
        printf("%d\n", s[a[i]]);

    return 0;
}

