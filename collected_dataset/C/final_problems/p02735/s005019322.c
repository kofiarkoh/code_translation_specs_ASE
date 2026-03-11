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

ll dp[110][110];
char s[110][110];
int hidari[110][110];//左からつなげて達成できる(右へ伸ばせる)
int ue[110][110];//上からつなげて達成できる(下へ伸ばせる)
int main(){
	int h,w;
	scanf("%d%d",&h,&w);
	rep(i,0,h)scanf("%s",s[i]);

	if(s[0][0]=='#'){
		dp[0][0]=1;
		hidari[0][0]=1;
		ue[0][0]=1;
	}
	rep(j,1,w){
		dp[0][j]=dp[0][j-1];
		if(s[0][j]=='#'){
			if(s[0][j-1]=='.'){
				dp[0][j]++;
				hidari[0][j]=1;
				ue[0][j]=1;
			}else{
				hidari[0][j]=1;
			}
		}else{
			hidari[0][j]=1;
			ue[0][j]=1;
		}
	}
	rep(i,1,h){
		dp[i][0]=dp[i-1][0];
		if(s[i][0]=='#'){
			if(s[i-1][0]=='.'){
				dp[i][0]++;
				hidari[i][0]=1;
				ue[i][0]=1;
			}else{
				ue[i][0]=1;
			}
		}else{
			hidari[i][0]=1;
			ue[i][0]=1;
		}
	}
	
	rep(i,1,h)rep(j,1,w){
		if(s[i][j]=='.'){
			dp[i][j]=min(dp[i-1][j],dp[i][j-1]);
			hidari[i][j]=1;
			ue[i][j]=1;
		}else{
			//上からくるとき
			int uuu=dp[i-1][j]+(!ue[i-1][j]||s[i-1][j]=='.');
			//左からくるとき
			int lll=dp[i][j-1]+(!hidari[i][j-1]||s[i][j-1]=='.');
			if(uuu<lll){
				dp[i][j]=uuu;
				ue[i][j]=1;
			}else if(uuu>lll){
				dp[i][j]=lll;
				hidari[i][j]=1;
			}else{
				dp[i][j]=uuu;
				ue[i][j]=1;
				hidari[i][j]=1;
			}
		}
	}
//	rep(i,0,h){rep(j,0,w)printf("%d",dp[i][j]);puts("");}
	printf("%d\n",dp[h-1][w-1]);
}