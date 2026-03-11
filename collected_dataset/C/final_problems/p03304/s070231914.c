#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<limits.h>
#include<math.h>

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))

int main(void){
	long long int a,b,c;
	
	scanf("%lld%lld%lld",&a,&b,&c);

	printf("%.10f\n",(double)(MAX(0,a-2*c)+MIN(a,2*c))*(b-1)/a/a);
	
	return 0;
}
