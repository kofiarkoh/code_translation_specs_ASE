#include <stdio.h>
#include <string.h>

int main(void)
{
    int N = 0;
    int K = 0;
    int A[20000] = {0};
    int B[20000] = {0};

    scanf("%d %d", &N, &K);
    int i = 0;
    int j = 0;
    int k = 0;

    for (i = 0; i < N; i++)
    {
        scanf("%d", &A[i]);
    }

    for (i = 0; i < K; i++)
    {
        memset(B, 0, sizeof(B));
        for (j = 0; j < N; j++)
        {
            for (k = j - A[j];
                 k <= j + A[j];
                 k++)
            {
                if (k >= 0)
                {
                    B[k]++;
                }
            }
        }
        for (j = 0; j < N; j++)
        {
            A[j] = B[j];
        }
    }
    for (i = 0; i < N; i++)
    {
        printf("%d ", A[i]);
    }
    printf("\n");
    return 0;
}
