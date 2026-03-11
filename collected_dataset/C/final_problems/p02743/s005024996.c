#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int main(void){
	long int a,b,c;
	long double ta,tb,tc;

	scanf("%ld %ld %ld",&a,&b,&c);
	ta=sqrtl(a);
	tb=sqrtl(b);
	tc=sqrtl(c);
	if(ta+tb-tc<0){
		printf("Yes");
	}else{
		printf("No");
	}
	return 0;
}
