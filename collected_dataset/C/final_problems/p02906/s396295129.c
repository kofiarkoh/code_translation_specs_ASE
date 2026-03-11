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

//union-find
#define UFLIMIT (1<<17)
ll unicnt[UFLIMIT+10];//正ならcnt、非正なら根のindex
void ufinit(int n){rep(i,0,n)unicnt[i]=1;}
int ufroot(int x){return unicnt[x]<=0?-(unicnt[x]=-ufroot(-unicnt[x])):x;}
int ufsame(int x,int y){return ufroot(x)==ufroot(y);}
void uni(int x,int y){if((x=ufroot(x))==(y=ufroot(y)))return;if(unicnt[x]<unicnt[y]){int t=x;x=y;y=t;}unicnt[x]+=unicnt[y];unicnt[y]=-x;}
#undef UFLIMIT

int a[100010],b[100010],c[100010];
int main(){
	int n,m,q;
	scanf("%d%d%d",&n,&m,&q);
	rep(i,0,q)scanf("%d%d%d",a+i,b+i,c+i);
	ufinit(n);
	int cyc=0;
	rep(i,0,q){
		if(!c[i]){
			if(ufsame(a[i],b[i])){
				puts("No");
				return 0;
			}
			uni(a[i],b[i]);
			n--;
			m--;
		}else{
			cyc=1;
		}
	}
	if(n==2&&cyc==1)puts("No");
	else puts(n-1+cyc<=m&&m<=n*(n-1)/2?"Yes":"No");
}
