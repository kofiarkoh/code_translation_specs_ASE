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

ll a[100010];
ll base[60];
int main(){
	int n;
	scanf("%d",&n);
	rep(i,0,n)scanf("%lld",a+i);
	ll sum=0;
	rep(i,0,n)sum^=a[i];
	
	int rank=0;
	rep(k,0,60){
		int idx=-1;
		rep(i,rank,n)if(bit(a[i],59-k)){
			idx=i;
			break;
		}
		if(idx!=-1){
//			printf("%d %d\n",59-k,idx);
			ll t=a[idx];a[idx]=a[rank];a[rank]=t;
			base[59-k]=t;
			rep(i,0,n)if(i!=rank&&bit(a[i],59-k))a[i]^=a[rank];
			rank++;
//			rep(i,0,n)printf("%d ",a[i]);puts("");
		}
	}
//	rep(i,0,60)printf("%d %d\n",i,base[i]);
	ll temp=0;
	for(int k=59;k>=0;k--){
		ll t=base[k];
		if((temp^base[k])+(sum^base[k])>=temp+sum){
			temp^=base[k];
			sum^=base[k];
		}
	}
	printf("%lld\n",temp+sum);
}