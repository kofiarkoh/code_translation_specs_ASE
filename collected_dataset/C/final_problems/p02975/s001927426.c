#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <limits.h>
#include <stdbool.h>

int yes(int *a, int *b, int N, int C)
{
    for (int i = 0; i < N; i++)
    {
        if (a[i] == C && (*b >> i) == 0x00)
        {
            *b |= 0x01 << i;
            //printf("%d\n", *b);
            return 1;
        }
    }
    return 0;
}

int check(int *a, int N)
{
    //printf("%d\n", N);
    for (int j = 1; j < N; j++)
    {
        int b = 0;
        int d = a[0];
        int e = a[j];
        int temp = 0;
        for (int i = 1; i < N; i++)
        {
            //printf("%d %d %d %d\n", d, e, d ^ e, j);
            if (i == N - 1)
            {
                return 1;
            }
            else if (yes(a, &b, N, d ^ e) == 1)
            {
                e ^= d;
                d ^= e;
                continue;
            }
            else
                break;
        }
    }
    return 0;
}

int main()
{
    int N;
    scanf("%d", &N);
    int *a = (int *)malloc(sizeof(int) * N);
    //char b[1000000001] = {0};
    for (int i = 0; i < N; i++)
    {
        scanf("%d", &a[i]);
        //b[a[i]] += 1;
    }
    if (check(a, N) == 1)
    {
        printf("Yes\n");
    }
    else
    {
        printf("No\n");
    }
    return 0;
}