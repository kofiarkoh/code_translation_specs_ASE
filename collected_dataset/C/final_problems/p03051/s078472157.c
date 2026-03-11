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



//*
//遅延セグ木ここから
//↓ここを変える
typedef struct sayouso{ll k;}sayouso;
typedef struct atai{ll a;}atai;
//↑ここを変える

typedef struct node{sayouso T;atai x;}node;
node lsegN[1<<21],*lseg;
ll lsegNUM,lsegk;

//↓ここから変える
//区間加算+区間sum(区間加算+1点変更+1点取得がほしいけどこれでできる)
sayouso id={0};
atai xx(atai x,atai y){
	atai ret;
	ret.a=x.a+y.a;
	return ret;
}
atai Tx(sayouso T,atai x){
	atai ret;
	ret.a=T.k+x.a;
	return ret;
}
sayouso TT(sayouso S,sayouso T){
	sayouso ret;
	ret.k=S.k+T.k;
	return ret;
}
sayouso fT(sayouso T,ll k){
	sayouso ret;
	ret.k=T.k<<k;
	return ret;
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


ll n;
int a[500010];
int cum[500010];
ll det0[1<<20|100];
ll det1[1<<20|100];
ll cnt1[1<<20|100];
int main(){
	scanf("%lld",&n);
	rep(i,0,n)scanf("%lld",a+i);
	rep(i,0,n)cum[i+1]=cum[i]^a[i];
	lseguse(1<<20);
	lseginit();
	//xを作りたいなら、累積和がx,0,x,0,……,となるように選ぶよりほかない
	//それぞれ連続する個数の積
	
	rep(i,1,n+1){
		if(cum[i]==0){
			sayouso T={1};
			lsegupdate(0,1<<20,T);
		}else{
			ll x=cum[i];
			ll zero=lsegcalc(x,x+1).a;
			if(zero){
				//前回までを精算
				det1[x]=(det1[x]+(det0[x]+1)*cnt1[x]%MOD)%MOD;
				det0[x]=(det0[x]+ det1[x]   *zero   %MOD)%MOD;
				//リセット
				cnt1[x]=1;
				sayouso T;
				T.k=-zero;
				lsegupdate(x,x+1,T);
			}else{
				cnt1[x]++;
			}
		}
	}
	
//	rep(i,1,n+1)printf("%d ",cum[i]);puts("");

	ll ANS=0;
	if(cum[n]==0){
		//なんでもいい　全部使える
		ANS=pom(2,lsegcalc(0,1).a-1,MOD);//0
		rep(x,1,1<<20){
			det1[x]=(det1[x]+(det0[x]+1)*cnt1[x]%MOD)%MOD;
			ll temp=det1[x];
			ANS=(ANS+temp)%MOD;
//			printf("%d ",temp);
		}
	}else{
		//cum[n]だけ使える
		ll x=cum[n];
		ANS=(det1[x]+1)%MOD;
	}
	printf("%lld",ANS);
}