#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define max(p,q)((p)>(q)?(p):(q))
#define INF ((1LL<<62)-(1LL<<31))

//セグ木ここから
//*
//↓ここを変える
typedef struct atai{ll a;}atai;
atai xx(atai x,atai y){
	atai r;
	r.a=max(x.a,y.a);
	return r;
}
atai aid={0};
//↑ここを変える

int segNUM;
atai *segN,*seg;
void seguse(int n){
	segN=(atai*)calloc(2*n,sizeof(atai));
	segNUM=n;
	seg=segN+segNUM;
	rep(i,1,2*segNUM)segN[i]=aid;
}
//seg[]に値を与えてから初期化
void seginit(){for(int node=segNUM-1;node;node--)segN[node]=xx(segN[node*2],segN[node*2+1]);}
void segupdate(int node,atai x){
	//seg[node]をxに更新
	node+=segNUM;
	segN[node]=x;
	while(node/=2)segN[node]=xx(segN[node*2],segN[node*2+1]);
}

atai segcalcsub(int l,int r,int k,int cl,int cr){
	if(l>=r)return aid;
	//完全に含むとき
	if(l<=cl&&cr<=r)return segN[k];

	int cm=(cl+cr)/2;
	//左側だけ
	if(r<=cm)return segcalcsub(l,r,2*k  ,cl,cm);
	//右側だけ
	if(cm<=l)return segcalcsub(l,r,2*k+1,cm,cr);
	//両方
	return xx(segcalcsub(l,r,2*k,cl,cm),segcalcsub(l,r,2*k+1,cm,cr));
}
atai segcalc(int l,int r){return segcalcsub(l,r,1,0,segNUM);}
//セグ木ここまで
//*/

int n,q;
int bs(int L,int v){
	//f([L,r))>=vなる最小のr-1
	int l=L,r=n+2;//lはng,rはok
	while(r-l>1){
		int m=(l+r)/2;
		if(segcalc(L,m).a>=v)r=m;
		else l=m;
	}
	return r-1;
}

int main(){
	seguse(1<<17);
	scanf("%d%d",&n,&q);
	rep(i,1,n+1){
		int t;
		scanf("%d",&t);
		seg[i].a=t;
	}
	seg[n+1].a=INF;
	seginit();
	while(q--){
		int t;
		scanf("%d",&t);
		if(t==1){
			int i,v;
			scanf("%d%d",&i,&v);
			atai xx={v};
			segupdate(i,xx);
		}else if(t==2){
			int l,r;
			scanf("%d%d",&l,&r);
			printf("%lld\n",segcalc(l,r+1).a);
		}else{
			int l,v;
			scanf("%d%d",&l,&v);
			printf("%d\n",bs(l,v));
		}
	}
}