#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>
int main(void)
{
	//変数の宣言
	int x,y;
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	long int ans=0;
	int count=0;
	int temp,temp1,temp2;
	int max,min;
	int len;
	//データの読み込み
	
	scanf("%d %d",&x,&y);
	
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	
//	printf("計算部分終了\n");
	//出力
	
	if(abs(x-y)<=1){
		printf("Brown");
	}else{
		printf("Alice");
	}
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

