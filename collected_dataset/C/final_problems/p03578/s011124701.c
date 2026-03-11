#include<stdio.h>
#include<stdlib.h>
int asc(const void *a, const void *b)
{
    return *(int *)a - *(int *)b;
}
int main()
{
    int n;
    scanf("%d", &n);
    int i;
    int d[200005];
    for (i = 0; i < n; i++)
        scanf("%d", &d[i]);
    int m;
    scanf("%d", &m);
    int t[200005];
    for (i = 0; i < m; i++)
        scanf("%d", &t[i]);
    if (n < m)
    {
        printf("NO\n");
        return 0;
    }
    qsort(d, n, sizeof(int), asc);
    qsort(t, m, sizeof(int), asc);
    int j = 0;
    for (i = 0; i < m; i++)
    {
        while (d[j] != t[i])
        {
            j++;
            if (j == n)
            {
                printf("NO\n");
                return 0;
            }
        }
        d[j] = -1;
    }
    printf("YES\n");
    return 0;
}