#include <stdio.h>
#define MOD 1000000007

int main(){
	long a,b,x;
	scanf("%ld%ld%ld",&a,&b,&x);
	printf("%ld\n",(x+(x-b>0)*(x-b)/(a-b)%MOD*(b%MOD))%MOD);
}
