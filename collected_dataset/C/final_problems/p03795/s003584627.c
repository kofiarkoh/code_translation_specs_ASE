#include <stdio.h>
#include <stdlib.h>

int main()
{
    int n;
    scanf("%d", &n);
    printf("%d\n", 800 * n - 200 * (n / 15));
    return 0;
}