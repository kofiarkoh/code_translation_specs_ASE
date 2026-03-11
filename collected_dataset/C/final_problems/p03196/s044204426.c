#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

#define llong long long
#define fr(i,l,r) for(i=(l);i<(r);i++)
#define min(p,q) ((p)<(q)?(p):(q))
#define max(p,q) ((p)>(q)?(p):(q))
#define INF 1000000000000//10^12



int main(void)
{
	//変数の宣言
	llong int n,p;
	int num;
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	llong int ans=0;
	int count=0;
	int temp,temp1,temp2;
	int max,min;
	int len;
	int sum=0;
	int under,upper,middle;

	//データの読み込み
	scanf("%lld %lld",&n,&p);

	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	
	ans=1;
	
	for(i=2;i<1000000;i++){
		num=0;
		while(p%i==0){
			num++;
			p=p/i;
		}
		while(num>=n){
	//		printf("i=%d\n",i);
			ans=ans*i;
			num=num-n;
		}
	}
	
	
	
//	printf("計算部分終了\n");
	//出力
	
	printf("%lld",ans);
	
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

