#include<stdio.h>
#define MAX 100001


int main(){
	int n,ans,i,j;
	int a[MAX]={0};
	int b[MAX]={0};
	int max=-1;
	int idx=0;
	
	scanf("%d",&n);
	for(i=1;i<=n;i++){
		scanf("%d",&a[i]);
	}

	for(i=1;i<=n;i++){
		for(j=0;j<i;j++){
			if(a[i]>a[j] && b[j]>max)idx=j;
		}
		b[i]=b[idx]+1;
	}
	
	printf("%d\n",b[n]);

	
	return 0;
	
}
