#include <stdio.h>

int main()
{
    int n, a, b;
    int x, y, z;
    scanf("%d%d%d", &n, &a, &b);
    x = (a < b) ? a : b;
    y = (a + b - n < 0) ? 0 : a + b - n;
    printf("%d %d\n", x, y);
}