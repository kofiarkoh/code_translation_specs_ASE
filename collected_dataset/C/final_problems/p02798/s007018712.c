#include <stdio.h>
#include <stdint.h>
#define MAX(x,y) ((x) > (y) ? (x) : (y))
#define MIN(x,y) ((x) < (y) ? (x) : (y))
#define LL int64_t
#define REP(i,n) for (LL (i) = 0; (i) < (n); i++)
#define REPr(i,n) for (LL (i) = (n); (i) --> 0;)
#define INF 1e9
int DP[1<<18][18];
int A[18][2];

int bitcount(LL bits) {
    bits = (bits & 0x5555555555555555) + (bits >> 1 & 0x5555555555555555);
    bits = (bits & 0x3333333333333333) + (bits >> 2 & 0x3333333333333333);
    bits = (bits & 0x0f0f0f0f0f0f0f0f) + (bits >> 4 & 0x0f0f0f0f0f0f0f0f);
    bits = (bits & 0x00ff00ff00ff00ff) + (bits >> 8 & 0x00ff00ff00ff00ff);
    bits = (bits & 0x0000ffff0000ffff) + (bits >>16 & 0x0000ffff0000ffff);
    return (bits & 0x00000000ffffffff) + (bits >>32 & 0x00000000ffffffff);
}

int main() {
    int N;
    LL ans = INF;
    scanf("%d", &N);
    REP(i, N) scanf("%d", &A[i][0]);
    REP(i, N) scanf("%d", &A[i][1]);

    REP(i, 1<<N) REP(j, N) DP[i][j] = INF;
    REP(j, N) DP[1<<j][j] = 0;

    REP(i, 1<<N) {
        REP(j, N) {
            if (DP[i][j] >= INF) continue;
            int ic = bitcount(i);
            int a = A[j][(j^ic^1)&1];
            REP(k, N) {
                if (i&(1<<k) || a > A[k][(k^ic)&1]) continue;
                int t = ic - bitcount(((1<<k)-1)&i);
                DP[i|(1<<k)][k] = MIN(DP[i|(1<<k)][k], DP[i][j]+t);
            }
        }
    }

    REP(j, N) ans = MIN(ans, DP[(1<<N)-1][j]);
    if (ans >= INF) ans = -1;
    printf("%d\n", ans);
    return 0;
}