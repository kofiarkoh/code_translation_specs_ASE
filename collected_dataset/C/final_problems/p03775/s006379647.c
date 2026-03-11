#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define max(a,b) (a>b)?(a):(b)

int llintcmp(const void *a, const void *b) {
	long long c = *(double *)a, d = *(double *)b;
	if (c>d) return 1;
	else if (c==d) return 0;
	else return -1;
}

int digits(long long a) {
	long long p = a/10LL;
	int d = 1;
	
	while (p) {
		p /= 10LL;
		d++;
	}
	return d;
}

int main(void) {
	int d, i, j, k;
	long long N, p, q;
	long long div[10000];
	scanf("%lld", &N);

	div[0] = 1LL;
	j = 1;
	for (p=2;p*p<=N;p++) {
		if (N%p == 0) div[j++] = p;
	}
//	for (i=0;i<j;i++) printf("%lld\t", div[i]);
	if (j == 1) {
		d = digits(N);
	} else if (j%2) {
		for (k=j;k<2*j-1;k++) {
			div[k] = N/div[2*j-k-2];
		}
		qsort(div,2*j-1,sizeof(long long), llintcmp);
		d = digits(div[j]);
	} else {
		for (k=j;k<2*j;k++) {
			div[k] = N/div[2*j-k-1];
		}
		qsort(div,2*j,sizeof(long long), llintcmp);
		d = digits(max(div[j],div[j+1]));
	}

//	for (i=0;i<=2*j;i++) printf("%lld\t", div[i]);
	printf("%d\n", d);
	return 0;
}