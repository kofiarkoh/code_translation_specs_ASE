#include<stdio.h>
int main(){
	long long n;
	scanf("%lld",&n);
	long long s=1;
	while(n&s)s*=2;
	printf("%lld\n",s);
}
