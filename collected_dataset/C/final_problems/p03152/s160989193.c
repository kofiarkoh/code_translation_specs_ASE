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

ll a[1010];
ll b[1010];
int main(){
	ll n,m;
	scanf("%lld%lld",&n,&m);
	rep(i,0,n)scanf("%lld",a+i);
	rep(i,0,m)scanf("%lld",b+i);
	sortdown(a,n);
	sortdown(b,n);
	int i=0,j=0;
	ll ans=1;
	for(int s=n*m;s>0;s--){
		if(a[i]==s&&b[j]==s){
			i++;
			j++;			
		}else if(a[i]==s){
			ans=ans*j%MOD;
			i++;
		}else if(b[j]==s){
			ans=ans*i%MOD;
			j++;
		}else{
			ans=ans*(i*j-(n*m-s))%MOD;
		}
//	printf("%lld\n",ans);
	}
	printf("%lld\n",ans);
	return 0;
}