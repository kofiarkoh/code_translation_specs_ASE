#include <stdio.h>

int main(void)
{
    int n, a, b, total;
    scanf("%d", &n);
    if (n == 1)
        puts("Hello World");
    if (n == 2)
    {
        scanf("%d", &a);
        scanf("%d", &b);
        total = a + b;
        printf("%d", total);
    }
    return 0;
}