#include <stdio.h>
#include <stdlib.h>

const int Mod = 1000000007;

void merge_sort(long long x[], int n)
{
	static long long y[200001] = {};
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
	int i, N, K;
	long long A[200001];
	scanf("%d %d", &N, &K);
	for (i = 0; i < N; i++) scanf("%lld\n", &(A[i]));
	merge_sort(A, N);
	A[N] = 0;
	
	int k;
	long long ans;
	if (A[0] >= 0) {
		for (i = N - 1, ans = 1; i >= N - K; i--) ans = ans * A[i] % Mod;
	} else if (A[N-1] <= 0) {
		if (K % 2 == 1) {
			for (i = N - 1, ans = 1; i >= N - K; i--) ans = ans * llabs(A[i]) % Mod;
			if (ans != 0) ans = Mod - ans;
		} else {
			for (i = 0, ans = 1; i < K; i++) ans = ans * llabs(A[i]) % Mod;
		}
	} else {
		for (k = 0; k <= K && A[N-K+k] < 0; k += 2);
		if (k < K) {
			for (k += 2; k <= K && A[k-1] < 0 && A[k-2] * A[k-1] > A[N-K+k-2] * A[N-K+k-1]; k += 2);
			k -= 2;
		}
		ans = 1;
		for (i = 0; i < k; i++) ans = ans * llabs(A[i]) % Mod;
		for (i = N - 1; i >= N - K + k; i--) ans = ans * A[i] % Mod;
	}
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}