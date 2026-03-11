#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int pm(int x)
{
	if(x%2==1){
		return 1;
	}else{
		return -1;
	}
}


int compare_int(const void *a, const void *b)
{
    return *(int*)b - *(int*)a;
}

int main(void)
{
	//変数の宣言
	int n,m;
	int i,j,k;
	long int x[1010];
	long int y[1010];
	long int z[1010];
	long int sum[1010];
	long int temp;
	long int max=0;
	
	//データの読み込み
	scanf("%d %d",&n,&m);
	
	for(i=0;i<n;i++){
		scanf("%ld %ld %ld",&x[i],&y[i],&z[i]);
	}
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	
	
	for(i=0;i<(1<<n);i++){
		for(j=0;j<n;j++){
			sum[j]=x[j]*pm(i)+y[j]*pm(i/2)+z[j]*pm(i/4);
		}
		qsort(sum, n, sizeof(long int), compare_int);
		temp=0;
		for(j=0;j<m;j++){
			temp=temp+sum[j];
		}
		if(temp>max){
			max=temp;
		}
	}
	
	
	
	
	
//	printf("計算部分終了\n");
	
	//出力
	printf("%ld",max);
	
//	printf("結果の出力終了\n");
	
	return 0;
}