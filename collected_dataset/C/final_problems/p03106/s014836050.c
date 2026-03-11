#include<stdio.h>

int main(void){
	int A,B,K,i,count;
	scanf("%d %d %d",&A,&B,&K);
	
	for(count=0,i=100;;i--){
		if(A%i==0&&B%i==0) count++;
		if(count==K){
			printf("%d",i);
			return 0;
		}
	}
}