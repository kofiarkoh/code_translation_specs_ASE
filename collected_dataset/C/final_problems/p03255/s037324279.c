#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF ((1LL<<62)-(1LL<<40))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))

ll n,X;
ll x[2010];
ll dp[2010][2010];//dp[i][j]=ちょうどj往復でx[i]以降のゴミを片付ける最小コスト
int main(){
	scanf("%lld%lld",&n,&X);
	if(n>2000)return 0;
	rep(i,0,n)scanf("%lld",x+i);
	
	rep(i,0,n+1)rep(j,0,n+1)dp[i][j]=INF;
	
	dp[n-1][1]=x[n-1]*5+X+X;
	for(int i=n-2;i>=0;i--)for(int j=1;j<=n-i;j++){
		//拾うのは(n-i)個目のゴミ
		//持っているゴミの数がk-1個からk個に増えると、コストは2k+1増える
		dp[i][j]=min(dp[i+1][j]+((n-i+j-1)/j*2+1)*x[i],dp[i+1][j-1]+5*x[i]+X)+X;
	}
	ll ans=INF;
	rep(j,1,n+1)ans=min(ans,dp[0][j]);
	printf("%lld\n",ans);
}