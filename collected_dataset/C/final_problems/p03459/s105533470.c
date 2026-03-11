#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

#define llong long long
#define fr(i,l,r) for(i=(l);i<(r);i++)
#define min(p,q)((p)<(q)?(p):(q))


int main(void)
{
	//変数の宣言
	int n;
	static int t[100010];
	static int x[100010];
	static int y[100010];
	
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	int ans=0;
	int count=0;
	int sum=0;
	llong int temp,temp1,temp2;
	int max,min;
	//int len;
	//データの読み込み
	
	scanf("%d",&n);
	
	for(i=1;i<=n;i++){
		scanf("%d %d %d",&t[i],&x[i],&y[i]);
	}
	t[0]=0;
	x[0]=0;
	y[0]=0;
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	flag=1;
	for(i=1;i<=n;i++){
//		printf("%d %d\n",t[i]-t[i-1],abs(x[i]-x[i-1])+abs(y[i]-y[i-1]));
		if(t[i]-t[i-1]<abs(x[i]-x[i-1])+abs(y[i]-y[i-1])||(t[i]-t[i-1]-abs(x[i]-x[i-1])+abs(y[i]-y[i-1]))%2==1){
//			printf("i=%dでflag変化\n",i);
			flag=0;
		}
	}
	
//	printf("計算部分終了\n");
	//出力
	
	if(flag==0){
		printf("No");
	}else{
		printf("Yes");
	}
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

