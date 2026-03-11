
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>


int main(void)
{
	//変数の宣言
	
	int n,k;
	int temp=0;
	long int ans=0;
	int i,j;
	
	//データの読み込み
	scanf("%d %d",&n,&k);
	
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	
	for(i=1;i<n+1;i++){
		if(i>=k){
			temp=(i-k)*(n/i);
			if(n%i-k+1>0){
				if(k==0){
					temp=temp+n%i-k;
				}else{
					temp=temp+n%i-k+1;
				}
			}
//			printf("i=%d temp=%d\n",i,temp);
		}else{
			temp=0;
		}
		ans=ans+temp;
	}
	
	
//	printf("計算部分終了\n");
	
	//出力
	
	
	printf("%ld",ans);
	
	
//	printf("結果の出力終了\n");
	
	return 0;
}