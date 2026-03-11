#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define ll long long unsigned int
int main(void)
{
    int i, j, n;
    scanf("%d", &n);
    int k = 1;
    int count = 0;
    for (i = 0; i < n; i++)
    {
        scanf("%d", &j);
        if (j != k)
        {
            count++;
        }
        else
        {
            k++;
        }
    }
    if (count == n)
        count = 0xffffffff;
    printf("%d\n", count);
    return 0;
}