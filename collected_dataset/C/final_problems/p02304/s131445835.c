#include <stdio.h>
#include <stdlib.h>
#define ll int
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define INF ((1<<30)-(1<<15))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))

int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}

int uniquell(ll*a,ll*ans,int cnt){
	sortup(a,cnt);
	int c=0;
	ans[0]=a[0];
	rep(i,1,cnt)if(a[i]!=ans[c])ans[++c]=a[i];
	return c+1;
}
//座圧2
//前準備：
//圧縮したい配列aに-INFとINFを追加する
//uniquell(a,comp,n)でcompに昇順の配列を作る
ll cocompf(ll n,ll*comp,int compcnt){
	//comp[i]<=n<comp[i+1]なるiを返す
	ll l=0,r=compcnt;
	while(r-l>1){
		ll m=(l+r)/2;
		if(comp[m]<=n)l=m;
		else r=m;
	}
	return l;
}


//セグ木ここから
//*
//↓ここを変える
typedef struct atai{ll a;}atai;
atai xx(atai x,atai y){
	atai r;
	r.a=x.a+y.a;//sum
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

typedef struct Line{int x,y1,y2;}L;
L yoko[100010],tate[100010];
int yokoc(const void*p,const void*q){
	if((*(L*)p).x<(*(L*)q).x)return -1;
	return 1;
}

int tate1(const void*p,const void*q){
	if(tate[*(int*)p].y1<tate[*(int*)q].y1)return -1;
	return 1;
}
int tate2(const void*p,const void*q){
	if(tate[*(int*)p].y2<tate[*(int*)q].y2)return -1;
	return 1;
}

int yokocnt,tatecnt;
int x[200010],xxx;
int idx1[100010],idx2[100010];
int main(){
	int n;
	scanf("%d",&n);
	while(n--){
		int a,b,c,d;
		scanf("%d%d%d%d",&a,&b,&c,&d);
		if(a==c){
			tate[tatecnt].x =a;
			tate[tatecnt].y1=min(b,d);
			tate[tatecnt].y2=max(b,d);
			tatecnt++;
			x[xxx++]=a;
		}else{
			yoko[yokocnt].x =b;
			yoko[yokocnt].y1=min(a,c);
			yoko[yokocnt].y2=max(a,c);
			yokocnt++;
			x[xxx++]=a;
			x[xxx++]=c;
		}
	}
	x[xxx++]=-INF;
	x[xxx++]=INF;
	xxx=uniquell(x,x,xxx);
	rep(i,0,tatecnt)idx1[i]=idx2[i]=i;
	qsort(yoko,yokocnt,sizeof(L),yokoc);
	qsort(idx1,tatecnt,sizeof(int),tate1);
	qsort(idx2,tatecnt,sizeof(int),tate2);
	int tate1crr=0,tate2crr=0;
	long long ans=0;
	seguse(1<<18);
	rep(i,0,yokocnt){
		while(tate1crr<tatecnt&&tate[idx1[tate1crr]].y1<=yoko[i].x){
			//追加
			int cc=cocompf(tate[idx1[tate1crr]].x,x,xxx);
			atai x=seg[cc];
			x.a++;
			segupdate(cc,x);
			tate1crr++;
		}
		while(tate2crr<tatecnt&&tate[idx2[tate2crr]].y2<yoko[i].x){
			//削除
			int cc=cocompf(tate[idx2[tate2crr]].x,x,xxx);
			atai x=seg[cc];
			x.a--;
			segupdate(cc,x);
			tate2crr++;
		}
		//計算
		ans+=segcalc(cocompf(yoko[i].y1,x,xxx),cocompf(yoko[i].y2,x,xxx)+1).a;
	}
	printf("%lld\n",ans);
}
