#include <stdio.h>
#include <stdlib.h>
#include <string.h>
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

ll dp[25000010];

ll f(ll n,ll a,ll b,ll c,ll p,ll q,ll r){
	rep(i,0,n+1){
		dp[i]=i;
		if(i>=c)dp[i]=max(dp[i],dp[i-c]+r);
		if(i>=b)dp[i]=max(dp[i],dp[i-b]+q);
	}
	ll ret=0;
	rep(i,0,n+1)ret=max(ret,(i%a)+i/a*p+dp[n-i]);
	return ret;	
}

ll n,a,b,c,p,q,r;
int main(){
	scanf("%lld%lld%lld%lld%lld%lld%lld",&n,&a,&b,&c,&p,&q,&r);	
	n=f(n,a,b,c,p,q,r);
	n=f(n,p,q,r,a,b,c);
	printf("%lld",n);
}