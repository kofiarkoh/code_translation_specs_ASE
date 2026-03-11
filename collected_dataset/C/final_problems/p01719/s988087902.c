#include<stdio.h>
#include<stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define bit(n,m)((n>>m)&1)
#define max(p,q)((p)>(q)?(p):(q))

ll n,d;
int dp[100010];
int v[20],w[20];
int f(int x){
	rep(i,0,x+5)dp[i]=0;
	rep(i,0,n)w[i]=v[i];
	rep(i,0,n)scanf("%lld",v+i);
	rep(i,0,n)rep(j,0,x-w[i]+1)dp[j+w[i]]=max(dp[j+w[i]],dp[j]+v[i]);
	int ans=0;
	rep(i,0,x+1)ans=max(ans,dp[i]-i+x);
	return ans;
}

int main(){
	ll x;
	scanf("%lld%lld%lld",&n,&d,&x);
	rep(i,0,n)scanf("%lld",v+i);
	rep(i,0,d-1)x=f(x);
	printf("%lld\n",x);
}
