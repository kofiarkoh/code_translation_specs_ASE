#include <stdio.h>
#include <stdbool.h>

#define MAX 100
#define INFINITY (1LL << 32)

static int n;
static long long d[MAX][MAX];

static void floyd()
{
    for (int k = 0; k < n; k++) {
        for (int i = 0; i < n; i++) {
            if (d[i][k] == INFINITY)
                continue;
            for (int j = 0; j < n; j++) {
                if (d[k][j] == INFINITY)
                    continue;
                long long tmp = d[i][k] + d[k][j];
                d[i][j] = (d[i][j] < tmp) ? d[i][j] : tmp;
            }
        }
    }
}

int main()
{
    int e, u, v, c;
    scanf("%d %d", &n, &e);

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            d[i][j] = (i == j) ? 0 : INFINITY;
        }
    }

    for (int i = 0; i < e; i++) {
        scanf("%d %d %d", &u, &v, &c);
        d[u][v] = c;
    }

    floyd();

    bool negative = false;
    for (int i = 0; i < n; i++) {
        if (d[i][i] < 0)
            negative = true;
    }

    if (negative) {
        printf("NEGATIVE CYCLE\n");
    } else {
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (j > 0)
                    printf(" ");
                if (d[i][j] == INFINITY) {
                    printf("INF");
                } else {
                    printf("%lld", d[i][j]);
                }
            }
            printf("\n");
        }
    }

    return 0;
}
