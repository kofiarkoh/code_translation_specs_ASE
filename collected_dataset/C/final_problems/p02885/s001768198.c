#include<stdio.h>
int main(){
	int m,n;
	scanf("%d%d",&m,&n);
	if(m<=2*n)
		printf("0");
	else
		printf("%d",m-2*n);
	return 0;
}