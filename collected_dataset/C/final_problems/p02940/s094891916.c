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

int n;
char s[300010];
int main(){
	scanf("%d%s",&n,s);
	ll ans=1;
	int c1=0,c2=0;
	int pair=0,alone=0;
	//pairはc1とc2の組の個数、aloneはpairになってないc1の個数
	rep(i,0,3*n){
		if(pair==0){
			if(alone==0){
				//0種類
				alone++;
				c1=s[i];
			}else{
				//1種類
				if(s[i]==c1){
					alone++;
				}else{
					ans=ans*alone--%MOD;
					pair++;
					c2=s[i];
				}
			}
		}else{
			//2種類
			if(s[i]==c1){
				alone++;
			}else if(s[i]==c2){
				if(alone==0){
					int t=c1;c1=c2;c2=t;
					alone++;
				}else{
					ans=ans*alone--%MOD;
					pair++;
				}
			}else{
				ans=ans*pair--%MOD;
			}
		}
	}
	rep(i,1,n+1)ans=ans*i%MOD;
	printf("%lld",ans);
}