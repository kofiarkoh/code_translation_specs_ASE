#include <stdio.h>
#include <math.h>

int main(){
	long int n;
	unsigned long long int T[1001],A[1001];
	scanf("%ld",&n);
	long int i,j;
	for(i=0;i<n;i++){
		scanf("%lld %lld",&T[i],&A[i]);
	}
	unsigned long long int calc;
	for (i=1;i<n;i++){
		calc =0;
		calc =fmax(ceil((double)A[i-1]/A[i]),ceil((double)T[i-1]/T[i]));
			A[i] *=calc;
			T[i] *=calc;
	}
	printf("%lld\n",A[n-1]+T[n-1]);
	return 0;
}