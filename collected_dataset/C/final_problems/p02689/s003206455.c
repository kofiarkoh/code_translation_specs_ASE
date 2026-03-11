#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int M;
int N;
int *H;
int *A;
int *B;
int *T;

int main(void)
{
    int c;
    int d;
    int ans = 0;
    int check;
    scanf("%d", &N);
    scanf("%d", &M);
    H = (int *)malloc(sizeof(int) * N);
    A = (int *)malloc(sizeof(int) * M);
    B = (int *)malloc(sizeof(int) * M);
    T = (int *)malloc(sizeof(int) * N);
    for (c = 0; c < N; ++c)
    {
        scanf("%d", H + c);
        T[c] = c;
    }
    for (c = 0; c < M; ++c)
    {
        scanf("%d", A + c);
        scanf("%d", B + c);
        --A[c];
        --B[c];
    }
    for (c = 0; c < N; ++c)
    {
        check = 1;
        for (d = 0; d < M; ++d)
        {
            if (A[d] == c && H[B[d]] >= H[A[d]])
            {
                check = 0;
            }
            else if (B[d] == c && H[A[d]] >= H[B[d]])
            {
                check = 0;
            }
        }
        ans += check;
    }
    printf("%d", ans);
    free(A);
    free(B);
    free(H);
    free(T);
    return 0;
}