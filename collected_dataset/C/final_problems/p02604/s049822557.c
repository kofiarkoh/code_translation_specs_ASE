#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define ll              long long
#define rep(i,a,N)      for(ll i=(a);i<(N);i++)
#define INF             ((1LL<<62)-(1LL<<31))
#define max(p,q)        ((p)>(q)?(p):(q))
#define min(p,q)        ((p)<(q)?(p):(q))

int n;
int x[20],y[20],p[20];
ll ans[20];
ll dist[20];

void dfs(int k,int cnt){
	if(k==n){
		ll sum=0;
		rep(i,0,n)sum+=dist[i]*p[i];
		ans[cnt]=min(ans[cnt],sum);
		return;
	}
	//素通り
	int temp[20];
	rep(i,0,n)temp[i]=dist[i];
	dfs(k+1,cnt);
	//x座標
	rep(i,0,n)dist[i]=min(temp[i],abs(x[i]-x[k]));
	dfs(k+1,cnt+1);
	//y座標
	rep(i,0,n)dist[i]=min(temp[i],abs(y[i]-y[k]));
	dfs(k+1,cnt+1);
}
 
 
int main(){
	scanf("%d",&n);
	rep(i,0,n)scanf("%d%d%d",x+i,y+i,p+i);
	rep(i,0,n)dist[i]=min(abs(x[i]),abs(y[i]));
	rep(i,0,n)ans[i]=INF;
	dfs(0,0);
	rep(i,0,n+1)printf("%lld\n",ans[i]);
}