#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int tri_judge(int i, int j, int k);

int compare_int(const void *a, const void *b)
{
    return *(int *)b - *(int *)a;
}

int main()
{
    int n;
    scanf("%d", &n);
    int length[n];
    int i;
    for (i = 0; i < n; i++)
    {
        scanf("%d", &length[i]);
    }

    qsort(length, n, sizeof(int), compare_int);

    int j, k;
    int ans = 0;

    for (i = 0; i < n; i++)
    {
        for (j = i + 1; j < n; j++)
        {
            for (k = j + 1; k < n; k++)
            {
                if (tri_judge(length[i], length[j], length[k]))
                {
                    ans++;
                    //printf("%d,%d,%d\n", length[i], length[j], length[k]);
                }
                else
                {
                    break;
                }
            }
        }
    }
    printf("%d\n", ans);
    return 0;
}

int tri_judge(int i, int j, int k)
{
    if (i + j > k && j + k > i && k + i > j)
    {
        return 1;
    }
    else
    {
        return 0;
    }
}