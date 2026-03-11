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
#define MOD 998244353
//#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

ll n,x;
ll dp1[3010][6010];//dp[n][s]=0,1,2をn個でs
ll dp2[3010][6010];//dp[n][s]=1,2をn個でs
ll choose[3010][3010];
int main(){
	scanf("%lld %lld",&n,&x);
	dp1[0][0]=1;
	dp2[0][0]=1;
	for(int i=0;i<n;i++)for(int j=0;j<=2*i;j++){
		for(int k=0;k<3;k++)dp1[i+1][j+k]=(dp1[i+1][j+k]+dp1[i][j])%MOD;
		for(int k=1;k<3;k++)dp2[i+1][j+k]=(dp2[i+1][j+k]+dp2[i][j])%MOD;
	}
	
	choose[0][0]=1;
	for(int i=1;i<3005;i++){
		choose[i][0]=1;
		for(int j=1;j<=i;j++)choose[i][j]=(choose[i-1][j-1]+choose[i-1][j])%MOD;
	}
	
	ll ans=0;
	for(int i=0;i<x;i++)ans=(ans+dp1[n][i])%MOD;
//	printf("%lld\n",ans);
	
	//x-1-iをn-i個以下の1,2で埋める
	for(int i=2;;i+=2){
		if(x-i-1<0||n-i<0)break;
		for(int j=0;j<=n-i;j++)ans=(ans+dp2[j][x-i-1]*choose[n][n-i-j])%MOD;
	}
	
	if(x%2==1){
		for(int i=x;i<=n;i++)ans=(ans+choose[n][i])%MOD;
	}
	
	printf("%lld",ans);
}