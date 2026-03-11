
#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

#define ll long long
#define fr(i,l,r) for(i=(l);i<(r);i++)
#define min(p,q)((p)<(q)?(p):(q))

static int par[200010];//親
static int rank[200010];//木の深さ

//n要素で初期化
void init(int n){
	int i;
	for(i=0;i<n;i++){
		par[i]=i;
		rank[i]=0;
	}
}

//木の根を求める
int find(int x){
	if(par[x]==x){
		return x;
	}else{
		par[x]=find(par[x]);
		return par[x];
	}
}

//xとyの属する集合を併合
void unite(int x, int y){
	x=find(x);
	y=find(y);
	if(x==y) return;
	if(rank[x]<rank[y]){
		par[x]=y;
	}else{
		par[y]=x;
		if(rank[x]==rank[y])rank[x]++;
	}
}

static int par2[200010];//親
static int rank2[200010];//木の深さ

//n要素で初期化
void init2(int n){
	int i;
	for(i=0;i<n;i++){
		par2[i]=i;
		rank2[i]=0;
	}
}

//木の根を求める
int find2(int x){
	if(par2[x]==x){
		return x;
	}else{
		par2[x]=find2(par2[x]);
		return par2[x];
	}
}

//xとyの属する集合を併合
void unite2(int x, int y){
	x=find2(x);
	y=find2(y);
	if(x==y) return;
	if(rank2[x]<rank2[y]){
		par2[x]=y;
	}else{
		par2[y]=x;
		if(rank2[x]==rank2[y])rank2[x]++;
	}
}

struct tosi_t{
	int idnum;
	ll num;
}

;int compare_tosi(const void *a, const void *b) {
  struct tosi_t *A = (struct tosi_t *)a;
  struct tosi_t *B = (struct tosi_t *)b;
  if(A->num - B->num >0) return 1;
  if(A->num - B->num >0) return -1;
  return 0;
}

int main(void)
{
	//変数の宣言
	int n;
	//int k,l;
	int p,q,r,s;
	static struct tosi_t tosi[200010];
	int pr[200010];
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	int ans=0;
	int count=0;
	int temp,temp1,temp2;
	int max,min;
	int len;
	//データの読み込み
	
	init(200010);
	init2(200010);
	
	scanf("%d %d %d",&n,&k,&l);
	for(i=0;i<k;i++){
		scanf("%d %d",&p,&q);
		unite(p,q);
	}
	for(i=0;i<l;i++){
		scanf("%d %d",&r,&s);
		unite2(r,s);
	}
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	
	for(i=1;i<=n;i++){
		tosi[i].num=par[i]*1000000+par2[i];
		tosi[i].idnum=i;
	}
	
//	for(i=1;i<=n;i++){
//		printf("par[%d]=%d,par2[%d]=%d num=%d\n",i,par[i],i,par2[i],tosi[i].num);
//	}
	
	qsort(tosi, n, sizeof(struct tosi_t), compare_tosi);
	
//	for(i=1;i<=n;i++){
//		printf("par[%d]=%d,par2[%d]=%d num=%d\n",i,par[i],i,par2[i],tosi[i].num);
//	}
	
	i=1;
	j=1;
	while(j<n+1){
//		printf("i=%d,j=%d\n",i,j);
		if(tosi[i].num==tosi[j].num){
			i++;
			flag=1;
		}else{
			if(flag==1) count=i-j;
			pr[tosi[j].idnum]=count;
//			printf("tosi[j].idnum=%d,count=%d\n",tosi[j].idnum,count);
			j++;
			flag=0;
		}
	}
	
	for(i=1;i<=n;i++){
		if(i!=1) printf(" ");
		printf("%d",pr[i]);
	}
	
//	printf("計算部分終了\n");
	//出力
	
	
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

