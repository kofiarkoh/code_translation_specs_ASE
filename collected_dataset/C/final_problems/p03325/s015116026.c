#include<stdio.h>

int main()
{
    int a[10000], i, n, count=0;

    scanf("%d", &n);

    for (i = 0; i < n; i++)
        scanf("%d", &a[i]);

    for (i = 0; i < n; i++) {
        while (1)
            if (a[i] % 2 == 0 && a[i] != 0) {
                a[i] = a[i] / 2;
                count++;
            }
            else break;
    }

    printf("%d", count);

    return 0;
}