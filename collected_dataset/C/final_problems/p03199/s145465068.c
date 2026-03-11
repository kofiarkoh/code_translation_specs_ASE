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

ll po(ll a,int n){ll x=1;while(n--)x*=a;return x;}

typedef struct t{ll m,w,c;}T;
int tsort(const void*a,const void*b){
	T p=*(T*)a,q=*(T*)b;
	if(p.m<q.m)return -1;
	if(p.m>q.m)return  1;
	if(p.w<q.w)return -1;
	if(p.w>q.w)return 1;
	return 1;
}

ll x[4][2]={{1,0},{1,1},{2,2},{4,4}};
//x[i][j]=i個で和をjにする場合の数

T a[50010];
ll dp[100010][2];
int main(){
	int n,m;
	scanf("%d%d",&n,&m);
	rep(i,0,m){
		int x,y,c;
		scanf("%d%d%d",&x,&y,&c);
		a[i].m=max(x,y);
		a[i].w=x+y;
		a[i].c=c;
	}

	qsort(a,m,sizeof(T),tsort);

	//番兵
	a[m].m=a[m].w=a[m].c=INF;
	a[m+1].m=a[m+1].w=a[m+1].c=INF;

	ll ptr=0;//まだ見てない最小index

	//2以下は愚直に見る
	ll c2=3,s2=0;
	while(a[ptr].m<=2){
		c2--;
		if(a[ptr].c==1)s2^=1;
		ptr++;
	}
	if(ptr>0&&a[ptr-1].w==4){
		dp[2][a[ptr-1].c]=x[c2][a[ptr-1].c^s2];
	}else{
		dp[2][0]=x[c2][s2];
		dp[2][1]=x[c2][s2^1];
	}
	
	//3以上はwが同じ所が偶数で、右下3x3だけ調べる
	rep(i,3,n+1){
		ll cnt=i-3;

		while(a[ptr].m==i&&a[ptr].w<=2*i-3){
			if(a[ptr+1].m==i&&a[ptr].w==a[ptr+1].w){
				if(a[ptr].c!=a[ptr+1].c){
					puts("0");
					return 0;
				}
				cnt--;
				ptr+=2;
			}else{
				cnt--;
				ptr++;
			}
		}
		
		//残りcnt2個でs2^(i-1,i-1)
		ll cnt2=2,s2=0;
		while(ptr<m&&a[ptr].m==i&&a[ptr].w==2*i-2){
			cnt2--;
			if(a[ptr].c==1)s2^=1;
			ptr++;
		}
		
		//残りcnt3個でs3^(i-1,i-1)^(i,i)
		ll cnt3=2,s3=0;
		while(ptr<m&&a[ptr].m==i&&a[ptr].w==2*i-1){
			cnt3--;
			if(a[ptr].c==1)s3^=1;
			ptr++;
		}
		
		if(ptr<m&&a[ptr].m==i&&a[ptr].w==2*i){
			//右下が指定されている場合
			dp[i][a[ptr].c]=(dp[i-1][0]*x[cnt2][s2^0]*x[cnt3][a[ptr].c^0^s3]+dp[i-1][1]*x[cnt2][s2^1]*x[cnt3][a[ptr].c^1^s3])*pom(2,cnt,MOD)%MOD;
			ptr++;
		}else{
			dp[i][0]=(dp[i-1][0]*x[cnt2][s2^0]*x[cnt3][0^0^s3]+dp[i-1][1]*x[cnt2][s2^1]*x[cnt3][0^1^s3])*pom(2,cnt,MOD)%MOD;
			dp[i][1]=(dp[i-1][0]*x[cnt2][s2^0]*x[cnt3][1^0^s3]+dp[i-1][1]*x[cnt2][s2^1]*x[cnt3][1^1^s3])*pom(2,cnt,MOD)%MOD;
		}
	}
	printf("%lld",(dp[n][0]+dp[n][1])%MOD);
}