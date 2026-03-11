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

ll n;
ll a[3010];
ll dp[3010][3010];

ll f(int x,int y,int c){
	//このあとたろうが取れる得点の最大値
	if(dp[x][y])return dp[x][y];
	if(x+y==n)return 0;
	if(c==0){
		//自分の手番
		return dp[x][y]=max(f(x+1,y,1)+a[x],f(x,y+1,1)+a[n-1-y]);
	}else{
		return dp[x][y]=min(f(x+1,y,0),f(x,y+1,0));
	}
}

int main(){
	scanf("%lld",&n);
	ll s=0;
	rep(i,0,n){
		scanf("%lld",a+i);
		s+=a[i];
	}
	printf("%lld\n",2*f(0,0,0)-s);
	return 0;
}