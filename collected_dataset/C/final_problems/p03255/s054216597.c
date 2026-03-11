#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF ((1LL<<62)-(1LL<<40))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))

ll n,X;
ll x[200010];
ll cumx[200010];
int main(){
	scanf("%lld%lld",&n,&X);
	rep(i,0,n)scanf("%lld",x+i);
	rep(i,0,n)cumx[i+1]=cumx[i]+x[i];
	
	ll ans=INF;
	//往復回数を全探索
	rep(j,1,n+1){
		//1なら19/17/15/13/11/ 9/ 7/ 5/ 5
		//2なら11/ 9, 9/ 7, 7/ 5, 5/ 5, 5
		//3なら 7, 7, 7/ 5, 5, 5/ 5, 5, 5　というかんじ
		ll temp=0,amp=3;
		for(int i=n;i>0;i-=j,amp+=2)temp+=(cumx[i]-cumx[max(0,i-j)])*amp;
		temp+=(cumx[n]-cumx[n-j])*2+j*X;
		ans=min(ans,temp);
	}
	printf("%lld\n",ans+n*X);
}