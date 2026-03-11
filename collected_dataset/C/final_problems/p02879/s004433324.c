#include<stdio.h>
int main(void){
	int a,b,c;
	scanf("%d",&a);
	scanf("%d",&b);
	if(a<=9&&b<=9){
		c=a*b;
		printf("%d",c);
	}else{
		printf("-1");
	}
	return 0;
}