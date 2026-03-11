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

void mebiusu_henkan(int N,ll*dp){
	//積集合で条件が与えられたとき、和集合の条件を返す
	dp[0]=0;
	rep(k,0,N)rep(j,0,1<<N)if(bit(j,k))dp[j]-=dp[j^(1<<k)];
	rep(j,0,1<<N)if(!__builtin_parity(j))dp[j]*=-1;
}

int n,k,and,or;
int aa[99];
int a[99];
ll dp[1<<18];
ll choose[60][60];
ll coe[60];
#define N 18

int main(){
	scanf("%d%d%d%d",&n,&k,&and,&or);
	rep(i,0,n)scanf("%d",aa+i);

	//10
	rep(k,0,N){
		if(bit(and,k)==1&&bit(or,k)==0){
			puts("0");
			return 0;
		}
	}
	
	//00,11
	{
		int cnt=0;
		rep(i,0,n){
			int mask=~(and^or);
			if((mask&and)==(mask&aa[i]))a[cnt++]=aa[i];
		}
		n=cnt;
	}

	choose[0][0]=1;
	rep(i,1,55){
		choose[i][0]=1;
		rep(j,1,i+1)choose[i][j]=choose[i-1][j-1]+choose[i-1][j];
		rep(j,1,k+1)coe[i]+=choose[i][j];
	}
	
	//01
	rep(mask,1,1<<N){
		int hoge=mask&(and^or);
		//hogeのところが全部ダメ
		if(hoge!=mask)continue;
		ll aaaaa[99];
		rep(i,0,n)aaaaa[i]=a[i]&hoge;
		sortup(aaaaa,n);
		//rep(i,0,n)printf("%d ",aaaaa[i]);puts("");
		aaaaa[n]=-1;
		int cnt=0;
		rep(i,0,n){
			cnt++;
			if(aaaaa[i]!=aaaaa[i+1]){
				dp[mask]+=coe[cnt];
				cnt=0;
			}
		}
	}	
	//rep(i,0,1<<N)printf("%lld ",dp[i]);puts("");
	mebiusu_henkan(N,dp);
	//rep(i,0,1<<N)printf("%lld ",dp[i]);puts("");

	printf("%lld\n",coe[n]-dp[(1<<N)-1]);
}