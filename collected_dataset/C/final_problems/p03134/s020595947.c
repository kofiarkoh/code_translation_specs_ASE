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

ll n;
char s[2010];
ll dp[4010][4010];
int main(){
	scanf("%s",s);
	n=strlen(s);
	
	dp[0][0]=1;
	int c=0;
	rep(i,1,2*n+1){
		if(i-1<n)c+=s[i-1]-48;
		rep(j,0,2*i+1)if(j<=c&&i-j<=2*min(i,n)-c){
			if(j)dp[i][j]=(dp[i-1][j]+dp[i-1][j-1])%MOD;
			else dp[i][j]=dp[i-1][j];
//			printf("%d %d %d\n",i,j,dp[i][j]);
		}
	}
	printf("%lld",dp[2*n][c]);
}