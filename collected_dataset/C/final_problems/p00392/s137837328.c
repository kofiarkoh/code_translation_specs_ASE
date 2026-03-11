#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define MAX 100005
int N;
int a[MAX], b[MAX];
char f[MAX];

char notPrime[MAX] = { 1,1,0,0,1 };
void sieve(int max)
{
	int i, j, b = (int)sqrt(max);
	for (i = 2; i < max; i += 2) notPrime[i] = 1;
	for (i = 3; i <= b; i += 2) {
		if (!notPrime[i]) {
			for (j = i * i; j < max; j += i) notPrime[j] = 1;
		}
	}
}

int cmp(const void *a, const void *b)
{
	return *(int *)a - *(int *)b;
}

int main()
{
	int i, j, k, max;

	scanf("%d", &N);
	max = 0;  for (i = 0; i < N; i++) {
	    scanf("%d", &k);
		a[i] = b[i] = k;
		f[k] = 1;
		if (max < k) max = k;
	}
	qsort(b, N, sizeof(int), cmp);

	sieve(++max);
	for (i = N - 1; i >= 0; i--) {
		if (!notPrime[b[i]] && a[i] != b[i]) {
			for (j = b[i] << 1; j < max; j += b[i]) if (f[j]) break;
			if (j >= max) { puts("0"); return 0; }
		}
	}
	puts("1");
	return 0;
}
