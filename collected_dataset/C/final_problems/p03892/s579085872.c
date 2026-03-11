#include<stdio.h>
#include<stdlib.h>
int gcd(int a,int b){return b?gcd(b,a%b):a;}
int main(){
	int a,b,c,d;
	scanf("%d %d %d %d",&a,&b,&c,&d);
	a=abs(a-c);
	b=abs(b-d);
	c=gcd(a,b);
	a/=c;
	b/=c;
	printf("%d\n",(a+b-1)*c);
	return 0;
}
