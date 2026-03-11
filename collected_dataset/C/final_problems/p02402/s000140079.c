#include<stdio.h>

int main()
{
    signed long int a[10000], max, min, sum;
    int n, i;

    scanf("%d", &n);
    for (i = 0; i < n; i++) scanf ("%d", &a[i]);

    max = min = sum = a[0]; 

    for (i = 0; i < n - 1; i++) {
      if (max <  a[i + 1])
        max = a[i + 1];
      if (min >  a[i + 1])
        min = a[i + 1];

      sum = sum + a[i + 1];
      }

    printf ("%d %d %d\n", min, max, sum);

    return 0;
}