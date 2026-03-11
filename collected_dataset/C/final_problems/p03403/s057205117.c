#include <stdio.h>
#include <stdlib.h>

int main()
{
    long int n;
    scanf("%ld", &n);
    int ar[n+10], i, j, sum = 0, a, b, c;
    ar [0] = 0;
    for (i=1; i<=n; i++)
        scanf("%d", &ar[i]);
    ar[n+1] = 0;
    for (i=0; i<n; i++)
    {
        sum = 0;

        for (j=0; j<=n; j++)
        {
            a = ar[j];
            if(i==j)
                j++;
            b = ar[j+1];
            sum = sum + abs(b - a);
        }

        printf("%d\n", sum);
    }
    return 0;
}
