#include <stdio.h>
#include <stdlib.h>

int comp(const void *a, const void *b)
{
    return (*(int *)a - *(int *)b);
}

int main(void)
{
    int n, m;
    int a[100000];
    int i, j;
    int max;
    int t;
    
    scanf("%d%d", &n, &m);
    
    for (i = 0; i < m; i++){
        scanf("%d", &a[i]);
    }
    qsort(a, m, sizeof(int), comp);
#if 0
    for (i = 0; i < m; i++){
        printf("%d ", a[i]);
    }
#endif
    max = a[0] - 1;
    
    for (i = 0; i < m - 1; i++){
        t = (a[i + 1] - a[i]) / 2;
        if (max < t){
            max = t;
        }
    }
    t = n - a[m - 1];
    if (max < t){
        max = t;
    }
    
    printf("%d\n", max);
    
    return (0);
}

    
    
