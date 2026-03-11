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
//#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

ll MOD,p;
ll a[3010],b[3010];
ll temp[3010];
int main(){
	scanf("%lld",&MOD);
	p=MOD;
	rep(i,0,MOD){
		int y;
		scanf("%d",&y);
		if(y){
			rep(j,0,p+1)temp[j]=0;
			temp[p]=1;
			temp[1]=-1;
			//x^p-xをx-yで割る
			for(int j=p-1;j>=0;j--){
				b[j]=(b[j]+temp[j+1])%MOD;
				temp[j]=(temp[j]+temp[j+1]*i)%MOD;
			}
		}
	}
	rep(j,0,p)printf("%d ",(MOD-b[j])%MOD);
}