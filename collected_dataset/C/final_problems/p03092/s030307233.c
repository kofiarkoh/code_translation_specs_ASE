#include <stdio.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define min(p,q)((p)<(q)?(p):(q))

ll n,a,b;
ll p[5010];
ll ip[5010];//pの逆引き
ll memo[5010][5010];
ll f(int R,int K){
	if(R<=0||K<0)return 0;
	if(memo[R][K])return memo[R][K]-1;
	if(ip[K]>=R)return (memo[R][K]=f(R,K-1)+1)-1;
	
	ll cnt=0;
	rep(i,ip[K],R)if(p[i]<K)cnt++;
	
	ll ans;
	if(cnt){
		ll v1=f(ip[K],K-1)+cnt*b;
		ll v2=f(    R,K-1)+a;
		ans=min(v1,v2);
	}else{
		ans=f(R,K-1);
	}
	return (memo[R][K]=ans+1)-1;
}

int main(){
	scanf("%lld%lld%lld",&n,&a,&b);
	rep(i,0,n){
		ll t;
		scanf("%lld",&t);
		t--;
		p[i]=t;
		ip[t]=i;
	}
	printf("%lld",f(n,n-1));
}