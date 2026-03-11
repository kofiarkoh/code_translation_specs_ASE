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
#define bit(n,m)((n>>m)&1)
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
//#define MOD 998244353
#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

ll s[50][50];
int main(){
	int n,d;
	scanf("%d%d",&n,&d);
	if(d>30)return 0;
	
	rep(i,0,n){
		int x,y;
		scanf("%d%d",&x,&y);
		s[x%d][y%d]++;
	}
		
	ll ans=INF;
	rep(x,0,d)rep(y,0,d){//原点を総当たり
		ll q=0,r=0;
		rep(i,0,d)rep(j,0,d)if(s[(x+i)%d][(y+j)%d]){
			if(i==0&&j==0){
				ll temp=sqrt(s[(x+i)%d][(y+j)%d]-1);
				if(temp>=q){
					q=temp;
					r=max(r,max(i,j));
				}
			}else if(i==0||j==0){
				ll temp=sqrt(s[(x+i)%d][(y+j)%d]-1);
				while((temp+1)*(temp+1)<=s[(x+i)%d][(y+j)%d]-1)temp++;
				if(temp>=q){
					q=temp;
					r=max(r,s[(x+i)%d][(y+j)%d]<=temp*(temp+1)?0:max(i,j));
				}
			}else{
				ll temp=sqrt(s[(x+i)%d][(y+j)%d]-1);
				if(temp>=q){
					q=temp;
					r=max(r,max(i,j));
				}
			}
//			printf("%d %d %d %d %d\n",x,y,q,r,s[(x+i)%d][(y+j)%d]);
		}
//		printf("\n");
		ans=min(ans,q*d+r);
	}
	printf("%d",ans);
}