#include<stdio.h>
#include<stdlib.h>
#include<math.h>
int asc(const void *a, const void *b)
{
    return *(int *)a - *(int *)b;
}
int main()
{
    int n;
    scanf("%d", &n);
    int i;
    int a[200005];
    for (i = 0; i < n; i++)
        scanf("%d", &a[i]);
    for (i = 0; i < n; i++)
        a[i] -= i + 1;
    qsort(a, n, sizeof(int), asc);
    int b = a[n / 2];
    unsigned long long int ans = 0;
    for (i = 0; i < n; i++)
    {
        ans += abs(a[i] - b);
    }
    printf("%lld\n", ans);
    return 0;
}