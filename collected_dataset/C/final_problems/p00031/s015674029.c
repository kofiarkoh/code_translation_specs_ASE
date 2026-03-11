#include<stdio.h>

int n,x;

int main(){
	while(scanf("%d",&n)!=EOF){
		x=1;
		while(n!=0){
			if(n%2==1)printf("%d ",x);
			n=n/2;x=x*2;
		}
		printf("\n");
	}
	return 0;
}