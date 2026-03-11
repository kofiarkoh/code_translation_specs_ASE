#include <stdio.h>
#include <stdlib.h>
#define rep(i,l,r)for(int i=(l);i<(r);i++)

typedef struct tuple{int x,y;}T;

int qidx[500010];
T query[500010];//[x,y]
int qcmp(const void*p,const void*q){
	if(query[*(int*)p].y<query[*(int*)q].y)return -1;
	if(query[*(int*)p].y>query[*(int*)q].y)return 1;
	return 0;
}

//セグ木ここから
//*
//↓ここを変える
typedef struct atai{int a;}atai;
atai xx(atai x,atai y){
	atai r;
	r.a=x.a+y.a;
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


int diff[500010];//色c[i]の右端がdiff[i]からiに変わった
int c[500010];
int pos[500010];
int ans[500010];
int main(){
	int n,q;
	scanf("%d%d",&n,&q);
	rep(i,1,n+1)scanf("%d",c+i);
	rep(i,0,q)scanf("%d%d",&query[i].x,&query[i].y);
	
	//クエリをrの昇順にソート
	rep(i,0,q)qidx[i]=i;
	qsort(qidx,q,sizeof(int),qcmp);
	
	//右端を構築
	rep(i,1,n+1){
		diff[i]=pos[c[i]];
		pos[c[i]]=i;
	}
	
	//クエリに答える
	int*qi=qidx;
	seguse(1<<19);
	rep(r,1,n+1){
		atai x={1};
		segupdate(r,x);
		segupdate(diff[r],aid);
		while(qidx-qi<q&&query[*qi].y==r){
			ans[*qi]=segcalc(query[*qi].x,query[*qi].y+1).a;
			qi++;
		}
	}
	rep(i,0,q)printf("%d\n",ans[i]);
}