#include <stdio.h>
#include <stdlib.h>
#include <math.h>

long int n,m;

int main(void){
	scanf("%lld %lld", &n,&m);

	if(n==1 && m==1)	printf("1");
	else if(n==1 || m==1)	printf("%lld", n+m-3);
	else	printf("%lld", (n-2)*(m-2));

	return 0;
}
