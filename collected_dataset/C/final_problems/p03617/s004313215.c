

//#include "pch.h"
#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

#define ll long long
#define fr(i,l,r) for(i=(l);i<(r);i++)
#define min(p,q) ((p)<(q)?(p):(q))
#define max(p,q) ((p)>(q)?(p):(q))
#define INF 1000000000000//10^12




int main(void)
{
	//変数の宣言
	ll int n,q,h,s,d;
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	ll int ans=0;
	int count=0;
	int temp,temp1,temp2;
	int max,min;
	int len;
	int sum=0;
	
	
	//データの読み込み
	scanf("%lld %lld %lld %lld %lld",&q,&h,&s,&d,&n);
//	scanf_s("%lld %lld %lld %lld %lld",&q,&h,&s,&d,&n);



	//	printf("nは%dです\n", n);
	//	printf("データの読み込み終了\n");
	//実際の処理
	
	s=min(q*4,min(h*2,s));

	if(d<2*s){
		ans=n/2*d+n%2*s;
	}else{
		ans=n*s;
	}

	//	printf("計算部分終了\n");
	
	//出力
	
	printf("%lld",ans);
	
	//	printf("結果の出力終了\n");
	
	return 0;
}

