
#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

#define llong long long
#define fr(i,l,r) for(i=(l);i<(r);i++)
#define min(p,q)((p)<(q)?(p):(q))


int compare_int(const void *a, const void *b)
{
    return *(int*)a - *(int*)b;
}



int main(void)
{
	//変数の宣言
	llong int n;
	llong int m[40];
	llong int mtemp[40];
	static llong int a[100010][40];
	static llong int b[100010];
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	llong int ans=0;
	int count=0;
	int sum=0;
	llong int temp,temp1,temp2;
	int max,min;
	int len;
	//データの読み込み
	
	scanf("%lld %lld",&n,&temp);
	
	for(i=0;i<30;i++){
		if(temp!=0){
			m[i]=temp%2;
			temp=temp/2;
		}else{
			m[i]=0;
		}
	}
	
	
	for(i=0;i<n;i++){
		scanf("%lld %lld",&temp,&b[i]);
		for(j=0;j<30;j++){
			if(temp!=0){
				a[i][j]=temp%2;
				temp=temp/2;
			}else{
				a[i][j]=0;
			}
		}
	}
	
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	
	for(i=0;i<30;i++){
		if(m[i]==1){
			mtemp[i]=0;
			for(j=0;j<i;j++){
				mtemp[j]=1;
			}
		}else{
			mtemp[i]=0;
			for(j=0;j<i;j++){
				mtemp[j]=m[j];
			}
		}
		for(j=i+1;j<30;j++){
			mtemp[j]=m[j];
		}
		
		temp=0;
		for(j=0;j<n;j++){
			flag=1;
			for(k=0;k<30;k++){
				if(a[j][k]>mtemp[k]) flag=0;
			}
			if(flag==1){
				temp=temp+b[j];
			}
		}
		if(ans<temp) ans=temp;
	}
	
	
//	printf("計算部分終了\n");
	//出力
	
	printf("%lld",ans);
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

