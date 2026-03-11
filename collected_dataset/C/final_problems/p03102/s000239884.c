#include <stdio.h>

int main(void)
{
    int N, M, C, i;
    scanf("%d%d%d", &N, &M, &C);

    int b[M], a[N][M];

    for (i = 0; i < M; i++)
        scanf("%d", &b[i]);

    for (i = 0; i < N; i++)
    {
        for (int j = 0; j < M; j++)
        {
            scanf("%d", &a[i][j]);
        }
    }

    int ans = 0;
    for ( i = 0; i < N; i++)
    {
        int sum = 0;
        for (int j = 0; j < M; j++)
        {
            sum += a[i][j] * b[j];
        }
        if(sum + C > 0)
            ans++;
        }

        printf("%d\n", ans);

        return 0;
}