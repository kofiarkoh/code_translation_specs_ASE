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
#define MOD 998244353
//#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)


//階乗,二項係数
//*
ll fact[2000010];
ll invfact[2000010];
void makefact(int n,ll m){
	n=min(n,m-1);
	fact[0]=1;
	rep(i,0,n)fact[i+1]=fact[i]*(i+1)%m;
	invfact[n]=invp(fact[n],m);
	for(int i=n;i>0;i--)invfact[i-1]=invfact[i]*i%m;
}
ll choose(int n,int r,int m){return n<0||r<0||n-r<0?0:fact[n]*invfact[r]%m*invfact[n-r]%m;}
//*/

int main(){
	ll n,m;
	scanf("%lld%lld",&n,&m);
	makefact(2000005,MOD);
	ll ans=0;
	rep(i,m,m*3/2+1){
		//和がiになるようn個に振り分け
		ll aa=choose(i+n-1,n-1,MOD);
		//m以上は使えない
		ll bb=n*choose(i-m+n-1,n-1,MOD);
		//m未満しかないときは好きにできる
		ll cc=(aa-bb)%MOD*choose(n,m*3-i*2,MOD);
		//mが1つある(⇔i-mをn-1個に振り分け)
		ll dd=n*choose(i-m+n-2,n-2,MOD);
		//mを避けて好きに割り振れる
		ll ee=dd%MOD*choose(n-1,m*3-i*2,MOD);
		//足す
		ans=(ans+cc+ee)%MOD;
	}
	printf("%lld",(ans+MOD)%MOD);
}