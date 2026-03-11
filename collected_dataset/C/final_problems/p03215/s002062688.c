
#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>

int compare_long(const void *a, const void *b)
{
	long *A = (long *)a;
	long *B = (long *)b;
	if (*A < *B) return 1;
	if (*A > *B) return -1;
	return 0;
}

int main(void)
{
	//変数の宣言
	int n,kosu;
	int a[1010];
	static long int suretu[600000];
	int njou;
	long int njou2;
	static int bl[600000];
	int keta[50];
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	long int ans=0;
	int count=0;
	long int temp,temp1,temp2;
	int max,min;
	int len;
	//データの読み込み
	
	scanf("%d %d",&n,&kosu);
	
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
	}
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	
	len=0;
	for(i=0;i<n;i++){
		temp=0;
		for(j=i;j<n;j++){
			temp=temp+a[j];
//			printf("j=%d,a[j]=%d,temp=%d\n",j,a[j],temp);
			suretu[len]=temp;
			len++;
		}
	}
	
//	for(i=0;i<len;i++){
//		printf("%d ",suretu[i]);
//	}
//	printf("\n");
	
	qsort(suretu, len, sizeof(long int), compare_long);
	
//	for(i=0;i<len;i++){
//	printf("%d ",suretu[i]);
//	}
//	printf("\n");
	
	njou=0;
	temp=1;
	while(flag==0){
		if(suretu[kosu-1]<temp*2){
			flag=1;
		}else{
			temp=temp*2;
			njou++;
		}
	}
	
//	printf("njou=%d\n",njou);
	
	njou2=1;
	for(i=0;i<njou;i++){
		njou2=njou2*2;
	}
	
	keta[njou]=1;
	for(i=0;i<len;i++){
		if(suretu[i]>=njou2){
			bl[i]=1;
		}else{
			len=i;
			break;
		}
	}
	
	temp=0;
	for(i=njou-1;i>=0;i--){
		count=0;
		for(j=0;j<len;j++){
			if(bl[j]==1){
				temp=suretu[j];
				for(k=0;k<i;k++){
					temp=temp/2;
				}
				if(temp%2==1){
					count++;
				}
			}
		}
//		printf("kosu=%d,count=%d\n",kosu,count);
		if(count>=kosu){
//			printf("count>=kosu\n");
			keta[i]=1;
			for(j=0;j<len;j++){
				if(bl[j]==1){
					temp=suretu[j];
					for(k=0;k<i;k++){
						temp=temp/2;
					}
					if(temp%2==0){
						bl[j]=0;
					}
				}
			}
		}else{
			keta[i]=0;
		}
		
//		for(j=0;j<len;j++){
//			printf("%d ",bl[j]);
//		}
//		printf("keta[%d]=%d count=%d\n",i,keta[i],count);
	}
	
	for(i=njou;i>=0;i--){
//		printf("keta[%d]=%d\n",i,keta[i]);
		if(keta[i]==1){
			ans++;
		}
		if(i!=0) ans=ans*2;
	}
	
//	printf("計算部分終了\n");
	//出力
	
	
	printf("%d",ans);
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

