#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF ((1LL<<62)-(1LL<<31))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))
#define bit(n,m)(((n)>>(m))&1)
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
//#define MOD 998244353
#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

#define chmax(p,q)(p=max(p,q))
ll dp[3010][3010][4];
ll v[3010][3010];
int main(){
	int r,c,k;
	scanf("%d%d%d",&r,&c,&k);
	rep(i,0,k){
		int x,y,z;
		scanf("%d%d%d",&x,&y,&z);
		v[x-1][y-1]=z;
	}
	
	dp[0][0][1]=v[0][0];
	rep(i,0,r)rep(j,0,c){
		if(i){
			//上からくる
			chmax(dp[i][j][0],max(dp[i-1][j][0],max(dp[i-1][j][1],max(dp[i-1][j][2],dp[i-1][j][3]))));
			chmax(dp[i][j][1],dp[i][j][0]+v[i][j]);
		}
		if(j){
			//左からくる
			//拾わない
			rep(k,0,4)chmax(dp[i][j][k],dp[i][j-1][k]);
			//拾う
			rep(k,0,3)chmax(dp[i][j][k+1],dp[i][j-1][k]+v[i][j]);
		}
	}
	printf("%lld",max(dp[r-1][c-1][0],max(dp[r-1][c-1][1],max(dp[r-1][c-1][2],dp[r-1][c-1][3]))));
}