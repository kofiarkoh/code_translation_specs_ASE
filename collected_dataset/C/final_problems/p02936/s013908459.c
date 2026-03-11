#include <stdio.h>

int ans[200005];
int G[200005][700];
int ct_G[200005];
int queue[200005];
int judge[200005];

int main()
{
    int N, Q;
    scanf("%d%d", &N, &Q);

    for (int i = 0; i < N; ++i)
    {
        ans[i] = 0;
        ct_G[i] = 0;
    }

    int a, b;
    for (int i = 0; i < N-1; ++i)
    {
        scanf("%d%d", &a, &b);
        G[a-1][ct_G[a-1]] = b-1;
        G[b-1][ct_G[b-1]] = a-1;
        ct_G[a-1] = ct_G[a-1] + 1;
        ct_G[b-1] = ct_G[b-1] + 1;
    }

    int p, x;
    for (int i = 0; i < Q; ++i)
    {
        scanf("%d%d", &p, &x);
        ans[p-1] = ans[p-1] + x;
    }

    judge[0] = 1;
    for (int i = 1; i < N-1; ++i) judge[i] = 0;
    int head = 1;
    int tail = 0;
    while (tail < head)
    {
        int a = queue[tail];
        tail++;
        for (int i = 0; i < ct_G[a]; ++i)
        {
            if (judge[G[a][i]] == 0)
            {
                judge[G[a][i]] = 1;
                ans[G[a][i]] = ans[G[a][i]] + ans[a];
                if (ct_G[a] > 0)
                {
                    queue[head] = G[a][i];
                    head++;
                }
            }
        }
    }

    for (int i = 0; i < N; ++i) printf("%d ", ans[i]);

    return 0;
}
