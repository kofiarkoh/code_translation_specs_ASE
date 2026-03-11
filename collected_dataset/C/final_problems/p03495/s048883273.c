#include <stdio.h>
#include <stdlib.h>

int int_sort(const void *a, const void *b);
int int_sort2(const void *a, const void *b);
int main(void)
{
    int N, K, i, j=0, A[200000], c[200000],tmp,sum=0;

    scanf("%d %d", &N, &K);

    for (i = 0; i < N;i++){
        c[i] = 0;
    }

    for (i = 0; i < N; i++)
    {
        scanf("%d", &A[i]);
        }

    qsort(&A, N, sizeof(int), int_sort);
    /*ここまででソート完了*/

    tmp = A[0];
    for (i = 0; i < N; i++)
    {
        if(tmp!=A[i]){
            j++;
            tmp = A[i];
        }
        c[j]++;
    }

    qsort(&c, N, sizeof(int), int_sort2);

    for (i = 0; i < K;i++){
        sum += c[i];
    }

    printf("%d\n", N-sum);

    return 0;
}
int int_sort(const void *a, const void *b)
{
    /* 引数はvoid*型と規定されているのでint型にcastする */
    if (*(int *)a < *(int *)b)
    {
        return -1;
    }
    else if (*(int *)a == *(int *)b)
    {
        return 0;
    }
    return 1;
}
int int_sort2(const void *a, const void *b)
{
    /* 引数はvoid*型と規定されているのでint型にcastする */
    if (*(int *)a < *(int *)b)
    {
        return 1;
    }
    else if (*(int *)a == *(int *)b)
    {
        return 0;
    }
    return -1;
}