#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define INF ((1LL<<62)-(1LL<<31))
#define f(x1,y1,x2,y2)(a[x2][y2]-a[x1][y2]-a[x2][y1]+a[x1][y1])
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}

int uniquell(ll*p,ll*ans,int cnt){
	ll*a=malloc(sizeof(ll)*cnt);
	memcpy(a,p,sizeof(ll)*cnt);
	sortup(a,cnt);
	int c=0;
	ans[0]=a[0];
	rep(i,1,cnt)if(a[i]!=ans[c])ans[++c]=a[i];
	return c+1;
}

//座圧2
//uniquell(a,comp,n)でcompに昇順の配列を作る
ll cocompf(ll n,ll*comp,ll compcnt){
	//comp[i]<=n<comp[i+1]なるiを返す
	ll l=0,r=compcnt;
	while(r-l>1){
		ll m=(l+r)/2;
		if(comp[m]<=n)l=m;
		else r=m;
	}
	return l;
}

int a[5010][5010];
ll x[5010],y[5010];
ll cx[5010],cy[5010];

int main(){
	int n,m;
	scanf("%d%d",&n,&m);
	rep(i,0,n)scanf("%lld%lld",x+i,y+i);
	x[n]=y[n]=-INF;
	x[n+1]=y[n+1]=INF;
	int xcnt=uniquell(x,cx,n+2);
	int ycnt=uniquell(y,cy,n+2);
	
	rep(i,0,n){
		int pp=cocompf(x[i],cx,xcnt)+1;
		int qq=cocompf(y[i],cy,ycnt)+1;
		a[pp][qq]++;
	}
	rep(i,0,xcnt+5)rep(j,0,ycnt+5)a[i][j+1]+=a[i][j];
	rep(j,0,ycnt+5)rep(i,0,xcnt+5)a[i+1][j]+=a[i][j];
	
	rep(_,0,m){
		ll p,q,r,s;
		scanf("%lld%lld%lld%lld",&p,&q,&r,&s);
		int x1=cocompf(p-1,cx,xcnt)+1;
		int y1=cocompf(q-1,cy,ycnt)+1;
		int x2=cocompf(r,cx,xcnt)+1;
		int y2=cocompf(s,cy,ycnt)+1;
		printf("%lld\n",f(x1,y1,x2,y2));
	}
}

