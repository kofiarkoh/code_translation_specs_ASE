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

char s[200010];
int a[200010];

//嘘貪欲
int main(){
	scanf("%s",s);
	int n=strlen(s);
	for(int i=n-1;i>=0;i--){
		a[i]=a[i+2]+s[i]-'0';
	}
	
	ll ans=a[0];
	ll cnt=0;
	int pos=0;
	repp(i,0,n,2){
		if(s[i]=='0'){
			//printf("%d %d\n",i,ans);
			while(i<n&&s[i]=='0'){
				i++;
				ans+=a[i]+cnt;
			}
		}
		cnt++;
		pos=i;
	}
	printf("%lld",ans+cnt*(cnt-1)+(pos!=n-1?cnt:0));
}


