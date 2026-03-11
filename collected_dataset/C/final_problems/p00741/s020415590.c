#include <stdio.h>
typedef long long ll;
#define rep(i,n) for(ll i=0; i<(ll)(n); i++)

int w, h;
int c[55][55];

void dfs(int x, int y) {
    c[x][y] = 0;
    rep(i, 3) rep(j, 3) {
        int nx = x-1+i, ny = y-1+j;
        if(nx == x && ny == y) continue;
        if(nx >= 0 && nx < w && ny >= 0 && ny < h) {
            if(c[nx][ny]) {
                dfs(nx, ny);
            }
        }
    }
}

int solve(void) {
    int ans = 0;
    rep(i, w) rep(j, h) {
        if(c[i][j]) {
            ans++;
            dfs(i, j);
        }
    }
    return ans; 
}

int main(){
    while(1){
        scanf("%d%d", &h, &w);
        if(w == 0 && h == 0) break;
        rep(i, w) rep(j, h){
            scanf("%d", &c[i][j]);
        }

        printf("%d\n", solve());
    }
}
