#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

#define llong long long
#define fr(i,l,r) for(i=(l);i<(r);i++)
#define min(p,q) ((p)<(q)?(p):(q))
#define max(p,q) ((p)>(q)?(p):(q))

static long int heap[100010];
int sz=-1;

void push(long int x){
	//自分のノード番号
	int i;
	int p;
	sz++;
	i=sz;
	
	while(i>0){
		//親のノード番号
		p=(i-1)/2;
		//もう逆転していないなら抜ける
		if(heap[p]<=x) break;
		//親のノードの数字を下ろして、自分は上に
		heap[i]=heap[p];
		i=p;
	}
	heap[i]=x;
}

int pop(){
	long int ret;
	int a,b;
	long int x;
	int i;
	//最小値
	ret=heap[0];
	//根に持ってくる値
	x=heap[sz];
	sz=sz-1;
	//根からおろしていく
	i=0;
	while(i*2+1<=sz){
		a=i*2+1;
		b=i*2+2;
		if(b<=sz&&heap[b]<heap[a]) a=b;
		//もう逆転していないなら終わり
		if(heap[a]>=x) break;
		//子の数字を持ち上げる
		heap[i]=heap[a];
		i=a;
	}
	heap[i]=x;
	
	return ret;
}

static long int heapup[100010];
int szup=-1;

void pushup(llong int x){
	//自分のノード番号
	int i;
	int p;
	szup++;
	i=szup;
	
	while(i>0){
		//親のノード番号
		p=(i-1)/2;
		//もう逆転していないなら抜ける
		if(heapup[p]>=x) break;
		//親のノードの数字を下ろして、自分は上に
		heapup[i]=heapup[p];
		i=p;
	}
	heapup[i]=x;
}

int popup(){
	llong int ret;
	int a,b;
	llong int x;
	int i;
	//最小値
	ret=heapup[0];
	//根に持ってくる値
	x=heapup[szup];
	szup=szup-1;
	//根からおろしていく
	i=0;
	while(i*2+1<=szup){
		a=i*2+1;
		b=i*2+2;
		if(b<=szup&&heapup[b]>heapup[a]) a=b;
		//もう逆転していないなら終わり
		if(heapup[a]<=x) {
			break;
		}
		//子の数字を持ち上げる
		heapup[i]=heapup[a];
		i=a;
	}
	heapup[i]=x;
	
	return ret;
}



int main(void)
{
	//変数の宣言
	int n;
	static llong int a[300010];
	static llong int lsum[100010];
	static llong int ldec[100010];
	static llong int rsum[100010];
	static llong int rdec[100010];
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	llong int ans=0;
	int count=0;
	llong int temp,temp1,temp2;
	int max,min;
	int len;
	//データの読み込み
	
	scanf("%d",&n);
	for(i=0;i<3*n;i++){
		scanf("%lld",&a[i]);
	}
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	lsum[0]=a[n];
	for(i=n+1;i<2*n;i++){
		lsum[i-n]=lsum[i-n-1]+a[i];
	}
	
	rsum[n-1]=a[2*n-1];
	for(i=2*n-2;i>=n;i--){
		rsum[i-n]=rsum[i-n+1]+a[i];
	}
	
	for(i=0;i<n;i++){
		push(a[i]);
	}
	
	
	for(i=n;i<2*n;i++){
		push(a[i]);
		if(i==n){
			ldec[i-n]=heap[0];
		}else{
			ldec[i-n]=ldec[i-n-1]+heap[0];
		}
		pop();
	}
	
	for(i=2*n;i<3*n;i++){
		pushup(a[i]);
	}
	
	for(i=2*n-1;i>=n;i--){
		pushup(a[i]);
		if(i==2*n-1){
			rdec[i-n]=heapup[0];
		}else{
			rdec[i-n]=rdec[i-n+1]+heapup[0];
		}
		popup();
	}
	
	for(i=0;i<=n;i++){
		if(i==0){
			ans=rdec[0]-rsum[0];
			temp=ans;
		}else if(i==n){
			temp=lsum[n-1]-ldec[n-1];
			if(temp>ans) ans=temp;
		}else{
			temp=lsum[i-1]-ldec[i-1]+rdec[i]-rsum[i];
			if(temp>ans) ans=temp;
		}
	}
	
	
	for(i=0;i<n;i++){
		ans=ans+a[i];
	}
	for(i=2*n;i<3*n;i++){
		ans=ans-a[i];
	}
	
//	printf("計算部分終了\n");
	//出力
	
	printf("%lld\n",ans);
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

