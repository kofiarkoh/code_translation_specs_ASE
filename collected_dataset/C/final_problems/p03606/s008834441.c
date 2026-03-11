#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

int main()
{
    int n, l[100000], r[100000], i;
    scanf("%d", &n);
    for (i=0; i<n; i++)
    {
        scanf("%d%d", &l[i], &r[i]);
    }
    int sum = 0;
    for (i=0; i<n; i++)
    {
        sum += r[i] - l[i] + 1;
    }
    printf("%d\n", sum);
    return 0;
}
