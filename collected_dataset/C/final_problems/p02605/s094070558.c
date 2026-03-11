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

typedef struct tuple{int x,y;}T;

int xcmp(const void*p,const void*q){
	T *pp=(T*)p,*qq=(T*)q;
	if(pp->x<qq->x)return-1;
	if(pp->x>qq->x)return 1;
	if(pp->y<qq->y)return-1;
	if(pp->y>qq->y)return 1;
	return 0;
}
int xcmp2(const void*p,const void*q){
	T *pp=(T*)p,*qq=(T*)q;
	if(pp->x<qq->x)return 1;
	if(pp->x>qq->x)return-1;
	return 0;
}
int ycmp(const void*p,const void*q){
	T *pp=(T*)p,*qq=(T*)q;
	if(pp->y<qq->y)return-1;
	if(pp->y>qq->y)return 1;
	if(pp->x<qq->x)return-1;
	if(pp->x>qq->x)return 1;
	return 0;
}

int cnt[4];
T temp[4][200010];

int hoge[800010],*aaa=hoge+400005;
int hoge2[800010],*bbb=hoge2+400005;

int main(){
	int n;
	scanf("%d",&n);
	rep(i,0,n){
		int x,y;
		char c;
		scanf("%d%d %c",&x,&y,&c);
		T*t;
		if(c=='U')t=temp[0]+cnt[0]++;
		if(c=='D')t=temp[1]+cnt[1]++;
		if(c=='R')t=temp[2]+cnt[2]++;
		if(c=='L')t=temp[3]+cnt[3]++;
		t->x=x;
		t->y=y;
	}
	int ans=1e9;
	
	//正面衝突1
	qsort(temp[0],cnt[0],sizeof(T),xcmp);
	qsort(temp[1],cnt[1],sizeof(T),xcmp);
	for(int l0=0,l1=0;l0<cnt[0];l0++){
		while(l1<cnt[1]&&xcmp(temp[0]+l0,temp[1]+l1)!=-1)l1++;
		if(l1<cnt[1]&&temp[1][l1].x==temp[0][l0].x)ans=min(ans,(temp[1][l1].y-temp[0][l0].y)*5);
	}
	//正面衝突2
	qsort(temp[2],cnt[2],sizeof(T),ycmp);
	qsort(temp[3],cnt[3],sizeof(T),ycmp);
	for(int l0=0,l1=0;l0<cnt[2];l0++){
		while(l1<cnt[3]&&ycmp(temp[2]+l0,temp[3]+l1)!=-1)l1++;
		if(l1<cnt[3]&&temp[3][l1].y==temp[2][l0].y)ans=min(ans,(temp[3][l1].x-temp[2][l0].x)*5);
	}
	
	//左から
	//aaa[i]=時刻0に(x,i)にいると下から衝突
	//bbb[i]=時刻0に(x,i)にいると上から衝突
	qsort(temp[2],cnt[2],sizeof(T),xcmp);
	for(int l0=0,l1=0,l2=0,x=0;x<=200000;x++){
		while(l2<cnt[2]&&temp[2][l2].x==x){
			aaa[temp[2][l2].y]=x+1;
			bbb[temp[2][l2].y]=x+1;
			l2++;
		}
		while(l0<cnt[0]&&temp[0][l0].x==x){
			if(aaa[temp[0][l0].y])ans=min(ans,(x+1-aaa[temp[0][l0].y])*10);
			l0++;
		}
		while(l1<cnt[1]&&temp[1][l1].x==x){
			if(bbb[temp[1][l1].y])ans=min(ans,(x+1-bbb[temp[1][l1].y])*10);
			l1++;
		}
		aaa++;
		bbb--;
	}
	
	//右から
	//aaa[i]=時刻0に(x,i)にいると下から衝突
	//bbb[i]=時刻0に(x,i)にいると上から衝突
	rep(i,0,800005)hoge[i]=hoge2[i]=0;
	aaa=hoge+400005,bbb=hoge2+400005;
	
	qsort(temp[3],cnt[3],sizeof(T),xcmp2);
	for(int l0=0,l1=0,l2=0,x=200000;x>=0;x--){
		while(l2<cnt[3]&&temp[3][l2].x==x){
			aaa[temp[3][l2].y]=x+1;
			bbb[temp[3][l2].y]=x+1;
			l2++;
		}
		while(l0<cnt[0]&&temp[0][l0].x==x){
			if(aaa[temp[0][l0].y])ans=min(ans,-(x+1-aaa[temp[0][l0].y])*10);
			l0++;
		}
		while(l1<cnt[1]&&temp[1][l1].x==x){
			if(bbb[temp[1][l1].y])ans=min(ans,-(x+1-bbb[temp[1][l1].y])*10);
			l1++;
		}
		aaa++;
		bbb--;
	}
	
	
	if(ans==1e9)puts("SAFE");
	else printf("%d",ans);
}