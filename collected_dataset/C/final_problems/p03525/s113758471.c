#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#define ll long long
#define rep(i,n) for(ll i=0;i<(n);i++)
#define max(p,q) ((p)>(q)?(p):(q))
#define min(p,q) ((p)<(q)?(p):(q))
#define chmax(a,b) ((a)=(a)>(b)?(a):(b))
#define chmin(a,b) ((a)=(a)<(b)?(a):(b))
#define abs(p) ((p)>=(0)?(p):(-(p)))
#define MOD 1000000007
ll power(ll a,ll b){ll r=1;rep(i,b){r*=a;}return r;}
#define swap(a,b) do{ll w=(a);(a)=(b);(b)=w;}while(0)

//your code here!
#define bit(subset_num,Rsift)(((subset_num)>>(Rsift))&1)

ll N, D[2][55];
ll dfs(ll time, ll subset){
    ll MIN=500, MIN2=500,x=power(2,time);
    rep(i, N+1) rep(j, i){
        chmin(MIN, abs(D[bit(subset,j)][j] - D[bit(subset,i)][i]));
        chmin(MIN2, abs(D[bit(subset+x,j)][j] - D[bit(subset+x,i)][i]));
    }
    ll ans = max(MIN, MIN2);
    if(time==N+1) return ans;
    else return max(ans, max(dfs(time+1, subset), dfs(time+1, subset+x)));
}

int main(void){
    scanf("%lld", &N);
    D[0][0]=D[1][0]=0;
    for(ll i=1; i<=N; i++) {
        scanf("%lld", &D[0][i]);
        if(D[0][i]) D[1][i] = 24 - D[0][i];
        else D[1][i] = D[0][i];
    }
    printf("%lld\n", dfs(0, 0));
    return 0;
}
