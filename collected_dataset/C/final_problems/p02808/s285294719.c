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


//階乗,二項係数
//*
ll fact[100010];
ll invfact[100010];
void makefact(int n,ll m){
	n=min(n,m-1);
	fact[0]=1;
	rep(i,0,n)fact[i+1]=fact[i]*(i+1)%m;
	invfact[n]=invp(fact[n],m);
	for(int i=n;i>0;i--)invfact[i-1]=invfact[i]*i%m;
}
ll choose(int n,int r,int m){return n<0||r<0||n-r<0?0:fact[n]*invfact[r]%m*invfact[n-r]%m;}
//*/

ll dp[30][1010];
ll n,k;
ll a[1010];
int main(){
	makefact(3000,MOD);
	scanf("%lld%lld",&n,&k);
	rep(i,0,k)scanf("%lld",a+i);
	dp[0][0]=fact[n];
	rep(i,0,k)rep(j,0,n+1){
		rep(m,0,n-j+1)dp[i+1][j+m]=(dp[i+1][j+m]+dp[i][j]*choose(n-m,a[i]-m,MOD)%MOD*invfact[m])%MOD;
	}
	printf("%lld",dp[k][n]);
}

