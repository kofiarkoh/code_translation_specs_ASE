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
#define MOD 998244353
//#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

ll n,m;
ll a[310][310];
int main(){
	scanf("%lld%lld",&n,&m);
	rep(i,0,n)rep(j,0,m)scanf("%lld",&a[i][j]);
	
	ll r=0;
	rep(j,0,m){
		ll t=-1;
		rep(i,0,n)if(a[i][j]){
			t=i;
			break;
		}
		if(t!=-1){
			r++;
			rep(i,0,n)if(i==t)rep(k,0,m)a[i][k]^=a[t][k];
		}
	}
	ll ans=pom(2,r-1,MOD)*(pom(2,r,MOD)-1+MOD)%MOD*pom(2,n-r,MOD)%MOD*pom(2,m-r,MOD)%MOD;
	printf("%lld",ans);
}