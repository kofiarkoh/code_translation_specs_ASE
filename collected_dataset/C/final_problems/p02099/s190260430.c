/*
 * ID: 3001
 * Proble: GPA janken, combination
 */

#include <stdio.h>

int main(void)
{
    double a[500000];
    int c[4001] = {};
    int s[4001] = {};

    int n;
    scanf("%d", &n);

    int i;
    for (i = 0; i < n; i++)
        scanf("%lf", a + i);

    for (i = 0; i < n; i++) 
        c[(int)(a[i] * 1000)]++;

    int k = n - 1;
    for (i = 4000; i >= 0 && k > 0; i--) {
        if (c[i] == 0)
            continue;
        s[i] = k * 3 - (c[i] - 1) * 2;
        k -= c[i];
    }

    for (i = 0; i < n; i++)
        printf("%d\n", s[(int)(a[i] * 1000)]);

    return 0;
}

