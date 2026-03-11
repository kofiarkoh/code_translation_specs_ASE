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
//#define MOD 998244353
#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

//座標圧縮
int cocomp(ll*a,int n){
	//1～cntに圧縮
	ll*b=(ll*)malloc(sizeof(ll)*n);
	memcpy(b,a,sizeof(ll)*n);
	sortup(b,n);
	int cnt=1;
	rep(r,1,n)if(b[r]!=b[cnt-1])b[cnt++]=b[r];
	rep(i,0,n){
		int l=0,r=cnt;
		while(r-l>1){int m=(l+r)/2;if(b[m]>a[i])r=m;else l=m;}
		a[i]=l+1;
	}
	free(b);
	return cnt;
}


//遅延セグ木ここから
//↓ここを変える
//fはフラグ、0ならid、1なら区間add、2なら区間更新
typedef struct sayouso{ll f,x;}sayouso;
typedef struct atai{ll a;}atai;
//↑ここを変える

typedef struct node{sayouso T;atai x;}node;
node lsegN[1<<21],*lseg;
ll lsegNUM,lsegk;

//↓ここから変える
//区間maxの計算、区間add、区間更新ができればよい
sayouso id={0,0};
atai xx(atai x,atai y){
	atai ret;
	ret.a=max(x.a,y.a);
	return ret;
}
atai Tx(sayouso T,atai x){
	atai ret;
	if(T.f==0)ret.a=x.a;
	if(T.f==1)ret.a=T.x+x.a;
	if(T.f==2)ret.a=T.x;
	return ret;
}
sayouso TT(sayouso S,sayouso T){
	if(S.f==0)return T;
	if(S.f==2)return S;
	T.x+=S.x;
	T.f=T.f?T.f:1;
	return T;
}
sayouso fT(sayouso T,ll k){
	return T;
}
//↑ここまで変える

//要素数(2ベキ)でこれを呼ぶ
void lseguse(ll n){
	lsegNUM=n;
	lseg=lsegN+lsegNUM;
	lsegk=0;while(n/=2)lsegk++;
}
//lseg[i].xに値を与えてから初期化
void lseginit(){
	for(ll i=lsegNUM-1;i>0;i--)lsegN[i].x=xx(lsegN[2*i].x,lsegN[2*i+1].x);
	rep(i,1,2*lsegNUM)lsegN[i].T=id;
}
void lsegupdatesub(ll l,ll r,sayouso T,ll i,ll cl,ll cr,ll ck){
	//disjointなとき
	if(cr<=l||r<=cl)return;
	//完全に含むとき
	if(l<=cl&&cr<=r){
		lsegN[i].T=TT(T,lsegN[i].T);
		return;
	}
	//どちらでもないとき
	//遅延伝播
	lsegN[2*i  ].T=TT(lsegN[i].T,lsegN[2*i  ].T);
	lsegN[2*i+1].T=TT(lsegN[i].T,lsegN[2*i+1].T);
	//再帰的に更新
	ll cm=(cl+cr)/2;
	lsegupdatesub(l,r,T,2*i  ,cl,cm,ck-1);
	lsegupdatesub(l,r,T,2*i+1,cm,cr,ck-1);
	//自身のnodeを更新
	lsegN[i].x=xx(Tx(fT(lsegN[2*i].T,ck-1),lsegN[2*i].x),Tx(fT(lsegN[2*i+1].T,ck-1),lsegN[2*i+1].x));
	lsegN[i].T=id;
}
void lsegupdate(ll l,ll r,sayouso T){lsegupdatesub(l,r,T,1,0,lsegNUM,lsegk);}
atai lsegcalcsub(ll l,ll r,ll i,ll cl,ll cr,ll ck){
	//完全に含むとき
	if(l<=cl&&cr<=r)return Tx(fT(lsegN[i].T,ck),lsegN[i].x);

	ll cm=(cl+cr)/2;
	//遅延伝播(変更はないので配るだけで良い)
	lsegN[2*i  ].T=TT(lsegN[i].T,lsegN[2*i  ].T);
	lsegN[2*i+1].T=TT(lsegN[i].T,lsegN[2*i+1].T);
	lsegN[i].x=Tx(fT(lsegN[i].T,ck),lsegN[i].x);
	lsegN[i].T=id;

	//左側だけ
	if(r<=cm)return lsegcalcsub(l,r,2*i  ,cl,cm,ck-1);
	//右側だけ
	if(cm<=l)return lsegcalcsub(l,r,2*i+1,cm,cr,ck-1);
	//両方
	return xx(lsegcalcsub(l,r,2*i,cl,cm,ck-1),lsegcalcsub(l,r,2*i+1,cm,cr,ck-1));
}
atai lsegcalc(ll l,ll r){return lsegcalcsub(l,r,1,0,lsegNUM,lsegk);}
//遅延セグ木ここまで
//*/

ll n,ans;
ll a[200010];
int main(){
	scanf("%lld",&n);
	rep(i,0,n)scanf("%lld",a+i);
	cocomp(a,n);
	
	lseguse(1<<18);
	int cr=0;
	rep(i,0,n+1){
		sayouso T;
		if(a[i]>cr){
			//[cr+1,a[i]]に+1
			T.f=1;
			T.x=1;
			lsegupdate(cr+1,a[i]+1,T);
		}else if(a[i]==cr){
			//a[i]に+1
			T.f=1;
			T.x=1;
			lsegupdate(a[i],a[i]+1,T);
		}else{
			//[a[i]+1,cr]のmaxを得たのち0に
			ll temp=lsegcalc(a[i]+1,cr+1).a;
			ans=max(ans,temp);
			T.f=2;
			T.x=0;
			lsegupdate(a[i]+1,cr+1,T);
			//a[i]に+1
			T.f=1;
			T.x=1;
			lsegupdate(a[i],a[i]+1,T);
		}
		cr=a[i];
		
	}
	printf("%lld",ans);
}