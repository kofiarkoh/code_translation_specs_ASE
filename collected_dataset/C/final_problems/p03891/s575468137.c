#include <stdio.h>

int main(void)
{
    int a = 0, b = 0, c = 0,
        d = 0, e = 0, f = 0,
        g = 0, h = 0, M = 0;

    scanf("%d %d %d", &a, &b, &M);

    g = 2*M - b;
    h = 2*M - a;
    f = 3*M - g - h;
    c = 2*M - f;
    e = 3*M - c - h;
    d = 2*M - e;

    printf("%d %d %d\n", a, b, c);
    printf("%d %d %d\n", d, M, e);
    printf("%d %d %d\n", f, g, h);

    return 0;
}
