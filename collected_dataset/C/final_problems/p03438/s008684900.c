#include <stdio.h>
#include <stdlib.h>

int main(void){
	int i;
	int n;
	scanf("%d",&n);
	int a[n],b[n],c[n];
	int sum=0,plus=0;
	for(i=0;i<n;i++){
		scanf("%d",a+i);
	}
	for(i=0;i<n;i++){
		scanf("%d",b+i);
		c[i]=a[i]-b[i];
		sum=sum+c[i];
		if(c[i]>0){
			plus=plus+c[i];
		}
	}
	if(plus+sum<=0){
		printf("Yes\n");
	}else{
		printf("No\n");
	}
	
	return 0;
}
