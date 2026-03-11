#include <stdio.h>
#include <stdlib.h>

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
	long long H, a[100001], b[100001];
	scanf("%d %lld", &N, &H);
	for (i = 0; i < N; i++) scanf("%lld %lld", &(a[i]), &(b[i]));

	long long max = 0, sum[100001] = {}, ans, count;
	for (i = 0; i < N; i++) if (a[i] > max) max = a[i];
	merge_sort(b, N);
	sum[N-1] = b[N-1];
	for (i = N - 2; i >= 0; i--) sum[i] = sum[i+1] + b[i]; 
	ans = (H - 1) / max + 1;
	for (i = 0; i < N; i++) {
		count = (H - sum[i] <= 0)? 0: (H - sum[i] - 1) / max + 1;
		count += N - i;
		if (count < ans) ans = count;
	}
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}