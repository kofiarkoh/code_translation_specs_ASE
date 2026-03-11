#include <stdio.h>

void merge_sort(long long x[], int n)
{
	static long long y[100001] = {};
	if (n == 1) return;
	merge_sort(&(x[0]), n/2);
	merge_sort(&(x[n/2]), (n+1)/2);
	
	int i, p, q;
	for (i = 0, p = 0, q = n/2; i < n; i++) {
		if (p >= n/2) y[i] = x[q++];
		else if (q >= n) y[i] = x[p++];
		else y[i] = (x[p] < x[q])? x[p++]: x[q++];
	}
	for (i = 0; i < n; i++) x[i] = y[i];
}

int main()
{
	int i, N;
	long long A[100001];
	scanf("%d", &N);
	for (i = 0; i < N; i++) scanf("%lld", &(A[i]));
	
	int j, max[2] = {};
	merge_sort(A, N);
	for (i = N - 1, j = 0; i > 0 && j < 2; i--) {
		if (A[i] == A[i-1])	max[j++] = A[i--];
	}
	
	printf("%lld\n", max[0] * max[1]);
	fflush(stdout);
	return 0;
}