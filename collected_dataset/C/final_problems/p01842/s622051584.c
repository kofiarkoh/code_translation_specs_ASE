#include<stdio.h>
#define rep(i,n) for(int i=0;(i)<(n);(i)++)
#define inf 1000000000
int MAX=52;
int n,m;
int a[52];
int b[52];
int dp[52][52][52][52][2][3];
int min(int x,int y){
    if(x<y) return x;
    else return y;
}
int max(int x,int y){
    if(x<y) return y;
    else return x;
}
int dfs(int l0, int r0, int l1, int r1, int turn, int pass, int p0, int p1) {
    //int ret = dp[l0][r0][l1][r1][turn][pass];
    //if (ret != inf && ret != -inf) return ret;
    int ret=dp[l0][r0][l1][r1][turn][pass];
    if (ret != inf && ret != -inf) return ret;
    //if(~dp[l0][r0][l1][r1][turn][pass])return dp[l0][r0][l1][r1][turn][pass];
    if (!turn) {
        ret=-inf;
        // stackに積む場合
        if (r0 < n) {
            if (a[r0] > 0) {
                // 得点カードを積む場合
                ret = max(ret, dfs(l0, r0 + 1, l1, r1, turn^1, 0, p0 + a[r0], p1));
            } else {
                // 妨害カードを積む場合
                ret = max(ret, dfs(l0, r0 + 1, l1, r1, turn^1, 0, p0, 0));
            }
        }
        // パスする場合
        if (l0 == r0 && l1 == r1) {
            // stackが空のとき
            if (pass == 0) {
                ret = max(ret, dfs(r0, r0, r1, r1, turn^1, 1, 0, 0));
            } else if (pass == 1) {
                ret = max(ret, 0ll);
            }
        } else {
            // stackが空でないとき
            ret = max(ret, dfs(r0, r0, r1, r1, turn^1, 0, 0, 0) + p0 - p1);
        }
    } else {
        ret=inf;
        // stackに積む場合
        if (r1 < m) {
            if (b[r1] > 0) {
                // 得点カードを積む場合
                ret = min(ret, dfs(l0, r0, l1, r1 + 1, turn^1, 0, p0, p1 + b[r1]));
            } else {
                // 妨害カードを積む場合
                ret = min(ret, dfs(l0, r0, l1, r1 + 1, turn^1, 0, 0, p1));
            }
        }
        // パスする場合
        if (l0 == r0 && l1 == r1) {
            // stackが空のとき
            if (pass == 0) {
                ret = min(ret, dfs(r0, r0, r1, r1, turn^1, 1, 0, 0));
            } else if (pass == 1) {
                ret = min(ret, 0ll);
            }
        } else {
            // stackが空でないとき
            ret = min(ret, dfs(r0, r0, r1, r1, turn^1, 0, 0, 0) + p0 - p1);
        }
    }
    
    return dp[l0][r0][l1][r1][turn][pass]=ret;
}
int main(void){
    scanf("%d%d",&n,&m);
    rep(i,n) scanf("%d",&a[i]);
    rep(i,m) scanf("%d",&b[i]);
    rep(i,MAX) rep(j,MAX) rep(k,MAX) rep(l,MAX) rep(p,2) rep(q,3) {
        dp[i][j][k][l][p][q] = (!p ? -inf : inf);
    }
    printf("%d\n",dfs(0,0,0,0,0,0,0,0));
}

