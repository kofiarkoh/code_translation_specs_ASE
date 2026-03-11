#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int i, j;

    int N, M;
    scanf("%d %d", &N, &M);

    int  *S, *T;
    S = calloc(N, sizeof(int));
    T = calloc(M, sizeof(int));

    for (i = 0; i < N; i++) {
        scanf("%d", &S[i]);
    }
    for (i = 0; i < M; i++) {
        scanf("%d", &T[i]);
    }

    long long **dp;
    long long **sum;
    dp = calloc(N, sizeof(long long *));
    sum = calloc(N, sizeof(long long *));
    for (i = 0; i < N; i++) {
        dp[i] = calloc(M, sizeof(long long));
        sum[i] = calloc(M, sizeof(long long));
    }


    dp[0][0] = (sum[0][0] = (S[0] == T[0]) ? 1 : 0);

    for (i = 1; i < M; i++) {
        dp[0][i] = (S[0] == T[i]) ? 1 : 0;
        sum[0][i] = sum[0][i-1] + dp[0][i];
    }

    for (i = 1; i < N; i++) {
        dp[i][0] = (S[i] == T[0]) ? 1 : 0;
        sum[i][0] = sum[i-1][0] + dp[i][0];
    }

    for (i = 1; i < N; i++) {
        for (j = 1; j < M; j++) {
            dp[i][j] = (S[i] == T[j]) ? sum[i-1][j-1] + 1 : 0;
            sum[i][j] = sum[i][j-1] + sum[i-1][j] - sum[i-1][j-1] + dp[i][j];
        }
    }

    printf("%lld\n", (sum[N-1][M-1] + 1) % 1000000007);

    return 0;

}