#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define lli long long int
#define min(a,b) ((a)<(b)?(a):(b))
#define max(a,b) ((a)>(b)?(a):(b))
#define true 1
#define false 0

int comp(const void *a,const void *b){
	return *(lli*)b - *(lli*)a;
}

int main(void){
	int i;
	int n;
	lli *a;
	int p;
	lli ans = 0;
	scanf("%d",&n);
	a = calloc(sizeof(lli),n);
	for(i=0;i<n;i++){
		scanf("%lld",&a[i]);
	}
	p = 1;
	i = 0;
	ans += a[i]-1;
	p++;
	for(i=1;i<n;i++){
		if(a[i]==p){
			p++;
			continue;
		}
		ans += (a[i]-1)/p;
	}
	printf("%lld\n", ans);


	return 0;
}