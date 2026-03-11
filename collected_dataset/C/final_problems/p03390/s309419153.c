
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
	int q;
	llong int a[110];
	llong int b[110];
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	llong int ans=0;
	int count=0;
	int sum=0;
	long int temp,temp1,temp2;
	int max,min;
	int len;
	//データの読み込み
	
	scanf("%d",&q);
	
	for(i=0;i<q;i++){
		scanf("%lld %lld",&a[i],&b[i]);
	}
	
//	for(i=0;i<q;i++){
//		printf("%lld %lld\n",a[i],b[i]);
//	}
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	for(i=0;i<q;i++){
//		printf("i=%d q=%d\n",i,q);
		if(a[i]==b[i]){
			printf("%lld\n",2*a[i]-2);
		}else if(abs(a[i]-b[i])==1){
			printf("%lld\n",a[i]+b[i]-3);
		}else{
			temp=(long int)sqrt(a[i]*b[i]);
			if(temp*(temp+1)<a[i]*b[i]){
				printf("%ld\n",temp*2-1);
			}else if(temp*(temp+1)==a[i]*b[i]){
				printf("%ld\n",temp*2-2);
			}else if(temp*temp==a[i]*b[i]){
				printf("%ld\n",temp*2-3);
			}else{
				printf("%ld\n",temp*2-2);
			}
		}
	}
//	printf("計算部分終了\n");
	//出力
	
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

