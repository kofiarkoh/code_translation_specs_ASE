#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>
#include <limits.h>

#define lli long long int
#define min(a,b) ((a)<(b)?(a):(b))
#define max(a,b) ((a)>(b)?(a):(b))
#define abs(a) ((a)<0?-(a):(a))



int main(void){
	int n,e,t;
	int *x;

	int i,j;
	lli res;

	scanf("%d%d%d",&n,&e,&t);
	x = calloc(sizeof(int),n);
	for(i=0;i<n;i++){
		scanf("%d",&x[i]);
	}
	i=0;
	res = x[i];
	while(i<n){
		//printf("%d\n", res);
		j=0;
		while(i+j < n && x[i] + t > x[i+j])j++;
		j--;
		res += max((x[i+j]-x[i])*2,t);
		j++;
		if(i+j == n){
			res += e - x[i];
			break;
		}
		res += x[i+j]-x[i];
		i=i+j;
	}
	printf("%lld\n", res);
    return 0;
}
