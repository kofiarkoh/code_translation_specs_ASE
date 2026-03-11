#include <stdio.h>
#include <stdlib.h>

#define MAXLEN 2*10000


int mycmp(const void* aa, const void* bb) {
	const int* a = aa;
	const int* b = bb;
	if (*a > *b) {
		return 1;
	} else if (*a < *b) {
		return -1;
	} else {
		return 0;
	}
}

int main(void) {
	unsigned int n;
	scanf("%u", &n);

	//long unsigned int as[MAXLEN], bs[MAXLEN];
	/*for (size_t i = 0; i < MAXLEN; i++) {
		as[i] = -1;
		bs[i] = -1;
	}*/

	long unsigned int* as = malloc(sizeof(long unsigned int) * n);
	long unsigned int* bs = malloc(sizeof(long unsigned int) * n);

	for (unsigned int i = 0; i < n; i++) {
		scanf("%lu %lu", as+i, bs+i);
	}

	qsort(as, n, sizeof(long unsigned int), mycmp);
	qsort(bs, n, sizeof(long unsigned int), mycmp);

	long unsigned int ans;
	
	if (n % 2 == 1) {
		long unsigned int a, b;
		a = as[(n-1)/2];
		b = bs[(n-1)/2];
		ans = b - a + 1;
	} else {
		long unsigned int a0, a1, b0, b1;
		a0 = as[(n-1)/2];
		a1 = as[n/2];
		b0 = bs[(n-1)/2];
		b1 = bs[n/2];
		ans = b0 - a0 + b1 - a1 + 1;
	}
	
	printf("%lu\n", ans);
	//fprintf(stderr, "OK!\n");
	return 0;
}
