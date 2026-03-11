#include <stdio.h>

void merge_sort(int x[], int n)
{
	static int y[100001] = {};
	if (n <= 1) return;
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
	int i, N, A[100001];
	long long M;
	scanf("%d %lld", &N, &M);
	for (i = 0; i < N; i++) scanf("%d", &(A[i]));
	merge_sort(A, N);
	
	if (M == 1) {
		printf("%d\n", A[N-1] * 2);
		fflush(stdout);
		return 0;
	} else if (M == (long long)N * N) {
		for (i = 0, M = 0; i < N; i++) M += A[i];
		printf("%lld\n", (__int128)M * M);
		fflush(stdout);
		return 0;
	}
	
	int j, l = A[0] * 2, r = A[N-1] * 2, m;
	long long count;
	while (l < r) {
		m = (l + r) / 2;
		count = 0;
		for (i = 0, j = N; i < N; i++) {
			for (; j > 0 && A[i] + A[j-1] >= m; j--);
			count += N - j;
		}
		
		if (count <= M) r = m;
		else l = m + 1;
	}
	
	__int128 ans = 0, sum = 0;
	for (i = 0, j = N, count = 0; i < N; i++) {
		for (; j > 0 && A[i] + A[j-1] >= l; j--) sum += A[j-1];
		ans += sum + (__int128)A[i] * (N - j);
		count += N - j;
	}
	printf("%lld\n", ans + (__int128)(M - count) * (l - 1));
	fflush(stdout);
	return 0;
}