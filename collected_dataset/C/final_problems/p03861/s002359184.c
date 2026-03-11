#include <stdio.h>
int main(void){
	long long int a,b,x;
	scanf("%lld %lld %lld",&a,&b,&x);
	if(a%x==0||b%x==0) printf("%lld",b/x-a/x+1);
	else printf("%lld\n",b/x-a/x);
	return 0;
}