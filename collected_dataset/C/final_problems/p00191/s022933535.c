#include <stdio.h>
#include <string.h>
#define REP(i,a,b) for (i=(a); i<(b); ++i)
#define rep(i,n) REP(i,0,n)
#define MAX(a, b) ((a) > (b) ? (a) : (b))
double a[101][101];
double memo[101][101][101];

int main(void)
{
    int n, m, i, j, k, l;
    double res;
    while (scanf("%d%d",&n,&m),n|m) {
        memset(memo, 0, sizeof(memo));
        rep(i, n) rep(j, n) scanf("%lf", &a[i][j]);
        res = 0;
        rep(i, n) rep(j, n) memo[0][i][j] = 1;
        REP(i, 1, m) rep(j, n) rep(k, n) rep(l, n)
            memo[i][j][k] = MAX(memo[i][j][k], memo[i-1][j][k] * a[k][l]);
        rep(i, n) rep(j, n) res = MAX(res, memo[m-1][i][j]);
        printf("%.2f\n", res);
    }

    return 0;
}