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
#define MOD 998244353
#define invp(a,p)pom(a,p-2,p)
#define add(x,y)(x=(x+y)%MOD)

int a[200],acnt;
char s[310];
int k;

#define S 310
#define K 22600
int dp[2][S][K];
int main(){
	scanf("%s %d",s,&k);
	int len=strlen(s);
	int acnt=0,cnt=0;
	rep(i,0,len){
		if(s[i]=='0'){
			a[acnt++]=cnt;
			cnt=0;
		}else{
			cnt++;
		}
	}
	if(cnt)a[acnt++]=cnt;
	while(cnt-1>=0&&a[cnt-1]==0)cnt--;
	int sum=0;
	{
		int cum=0;
		for(int i=acnt-1;i>=1;i--)cum+=sum+=a[i];
		k=min(k,cum);
		sum+=a[0];
	}
	
	if(k==0){
		puts("1");
		return 0;
	}
	
	//rep(i,0,acnt)printf("%d ",a[i]);puts("");
	
	int pre=0;
	dp[0][0][0]=1;
	rep(i,0,acnt){
		memset(dp[(i%2)^1],0,sizeof(int)*S*K);
		rep(s,pre,sum+1)rep(x,0,k+1)if(dp[i%2][s][x]){
			//printf("%d %d %d\n",i,s,x);
			rep(ss,max(pre+a[i],s),sum+1){
				//printf("%d %d %d %d\n",i,s,x,ss-s);
				if(x+max(0,ss-s-a[i])>k)break;
				add(dp[(i%2)^1][ss][x+max(0,ss-s-a[i])],dp[i%2][s][x]);
				//printf("%d %d %d %d\n",i,s,x,ss-s);
			}
		}
		pre+=a[i];
		
		//rep(s,0,5){rep(x,0,5)printf("%d ",dp[i+1][s][x]);puts("");}puts("");
	}
	
	ll ans=0;
	rep(i,0,k+1)add(ans,dp[acnt%2][pre][i]);
	printf("%d",ans);
}