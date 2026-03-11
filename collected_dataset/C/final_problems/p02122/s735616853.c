#include<stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define INF ((1LL<<62)-(1LL<<31))
#define min(p,q)((p)<(q)?(p):(q))


//*
//遅延セグ木ここから
//↓ここを変える
typedef struct sayouso{ll aa,ab,bb,ba;}sayouso;
typedef struct atai{ll a,b;}atai;
//↑ここを変える

typedef struct node{sayouso T;atai x;}node;
node *lsegN,*lseg;
ll lsegNUM,lsegk;

//↓ここから変える
sayouso sid={0,0,0,0};
atai aid={INF,INF};
atai xx(atai x,atai y){
	atai ret;
	ret.a=min(x.a,y.a);
	ret.b=min(x.b,y.b);
	return ret;
}
atai Tx(sayouso T,atai x){
	atai ret;
	if(T.aa)ret.a=T.aa;
	else if(T.ab)ret.a=x.b;
	else ret.a=x.a;
	if(T.bb)ret.b=T.bb;
	else if(T.ba)ret.b=x.a;
	else ret.b=x.b;
	return ret;
}
sayouso TT(sayouso S,sayouso T){
	sayouso ret;
	if(S.ab){
		ret.aa=T.bb?:0;
		ret.ab=!T.ba&&!T.bb;
	}else if(S.aa){
		ret.aa=S.aa;
		ret.ab=0;
	}else{
		ret.aa=T.aa;
		ret.ab=T.ab;
	}
	if(S.ba){
		ret.bb=T.aa?:0;
		ret.ba=!T.ab&&!T.aa;
	}else if(S.bb){
		ret.bb=S.bb;
		ret.ba=0;
	}else{
		ret.bb=T.bb;
		ret.ba=T.ba;
	}
	return ret;
}
sayouso fT(sayouso T,ll k){
	return T;
}
//↑ここまで変える

//要素数(2ベキ)でこれを呼ぶ
void lseguse(ll n){
	lsegN=(node*)calloc(2*n,sizeof(node));
	lsegNUM=n;
	lseg=lsegN+lsegNUM;
	lsegk=0;while(n/=2)lsegk++;
	rep(i,1,2*lsegNUM){
		lsegN[i].x=aid;
		lsegN[i].T=sid;
	}
}
//lseg[i].xに値を与えてから初期化
void lseginit(){
	for(ll i=lsegNUM-1;i>0;i--)lsegN[i].x=xx(lsegN[2*i].x,lsegN[2*i+1].x);
}
void lsegupdatesub(ll l,ll r,sayouso T,ll i,ll cl,ll cr,ll ck){
	if(l>=r)return;
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
	lsegN[i].T=sid;
}
void lsegupdate(ll l,ll r,sayouso T){lsegupdatesub(l,r,T,1,0,lsegNUM,lsegk);}
atai lsegcalcsub(ll l,ll r,ll i,ll cl,ll cr,ll ck){
	if(l>=r)return aid;
	//完全に含むとき
	if(l<=cl&&cr<=r)return Tx(fT(lsegN[i].T,ck),lsegN[i].x);

	ll cm=(cl+cr)/2;
	//遅延伝播(変更はないので配るだけで良い)
	lsegN[2*i  ].T=TT(lsegN[i].T,lsegN[2*i  ].T);
	lsegN[2*i+1].T=TT(lsegN[i].T,lsegN[2*i+1].T);
	lsegN[i].x=Tx(fT(lsegN[i].T,ck),lsegN[i].x);
	lsegN[i].T=sid;

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

int main(){
	int n;
	scanf("%d",&n);
	lseguse(1<<18);
	rep(i,1,n+1)scanf("%lld",&lseg[i].x.a);
	rep(i,1,n+1)scanf("%lld",&lseg[i].x.b);
	lseginit();
	int q;
	scanf("%d",&q);
	while(q--){
		int x,y,z;
		scanf("%d%d%d",&x,&y,&z);
		sayouso T={0,0,0,0};
		if(x==1){
			T.aa=z;
			lsegupdate(y,y+1,T);
		}else if(x==2){
			T.bb=z;
			lsegupdate(y,y+1,T);
		}else if(x==3){
			printf("%d\n",lsegcalc(y,z+1).a);
		}else if(x==4){
			printf("%d\n",lsegcalc(y,z+1).b);
		}else if(x==5){
			T.ab=1;
			lsegupdate(1,n+1,T);
		}else if(x==6){
			T.ba=1;
			lsegupdate(1,n+1,T);
		}
	}
}

