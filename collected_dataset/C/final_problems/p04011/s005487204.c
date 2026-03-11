#include<stdio.h>
int main(){
	int a,b,c,d;
	scanf("%d %d %d %d",&a,&b,&c,&d);
	printf("%d\n",(a<b?a:b)*c+(a<b?0:a-b)*d);
	return 0;
}