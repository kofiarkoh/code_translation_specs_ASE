#include <stdio.h>

int main(void){
	long long int a,b,x;

	scanf("%lld %lld %lld",&a,&b,&x);

	if(a == 0){
		printf("%lld\n",(b/x)+1);
	}
	else{
		printf("%lld\n",(b/x) - ((a-1)/x));
	}

	return 0;
}