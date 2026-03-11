#include <stdio.h>

int main(void)
{
    int n;
    scanf("%d", &n);

    int a, b;
    int i;
    long s = 0;
    for (i = 0; i < n; i++) {
        scanf("%d%d", &a, &b);
        if (a > b)
            s += (a - b);
        else if (a < b)
            s += a;
    }

    printf("%ld", s);
    return 0;
}