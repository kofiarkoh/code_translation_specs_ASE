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

#define add(x)(x=(x+t)%MOD)
ll L,R;
int f(int keta){
	//n桁目まで見て[L≦x][y≦R]
	ll dp[70][2][2]={};
	ll m=1LL<<(keta-1);
	ll M=(1LL<<keta)-1;
	if(M<L||R<m)return 0;
	ll l=max(L,m);
	ll r=min(R,M);
	
	
	dp[keta][0][0]=1;
	for(int n=keta;n>=1;n--){
		rep(i,0,2)rep(k,0,2){
		ll t=dp[n][i][k];
		//L x1 y1 R
		if(k||bit(r,n-1))add(dp[n-1][i|!bit(l,n-1)][k]);
		//L x0 y1 R
		if((i||!bit(l,n-1))&&(k||bit(r,n-1)))add(dp[n-1][i][k]);
		//L x0 y0 R
		if(i||!bit(l,n-1))add(dp[n-1][i][k|bit(r,n-1)]);
		}		
	}
	ll ans=0;
	rep(i,0,2)rep(k,0,2)ans=(ans+dp[0][i][k])%MOD;
	return ans;
}

int main(){
	scanf("%lld%lld",&L,&R);
	ll ans=0;
	rep(i,1,64)ans=(ans+f(i))%MOD;
	printf("%lld",ans);
}