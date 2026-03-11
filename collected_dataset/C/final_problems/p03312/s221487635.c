#include <stdio.h>
#include <stdlib.h>

int main()
{
	int i, N, A[200001];
	scanf("%d", &N);
	for (i = 1; i <= N; i++) scanf("%d", &(A[i]));
	
	int k[3], l, r, m;
	long long sum[200001], L, R, M;
	for (i = 1, sum[0] = 0; i <= N; i++) sum[i] = sum[i-1] + A[i];
	L = 0;
	R = sum[N];
	while (L < R) {
		M = (L + R) / 2;
		l = 1;
		r = N - 3;
		while (l < r) {
			m = (l + r) / 2;
			if (sum[m] < (sum[N] - sum[m] + 2) / 3 - M) l = m + 1;
			else r = m;
		}
		for (k[0] = l; k[0] <= N - 3 && sum[k[0]] <= (sum[N] - sum[k[0]]) / 3 + M; k[0]++) {
			l = k[0] + 1;
			r = N - 2;
			while (l < r) {
				m = (l + r) / 2;
				if (sum[m] - sum[k[0]] < (sum[N] - sum[m] + 1) / 2 - M || sum[k[0]] < (sum[N] - sum[m] + 1) / 2 - M || sum[m] - sum[k[0]] < sum[k[0]] - M) l = m + 1;
				else r = m;
			}
			for (k[1] = l; k[1] <= N - 2 && sum[k[1]] - sum[k[0]] <= (sum[N] - sum[k[1]]) / 2 + M && sum[k[0]] <= (sum[N] - sum[k[1]]) / 2 + M && sum[k[1]] - sum[k[0]] <= sum[k[0]] + M; k[1]++) {
				l = k[1] + 1;
				r = N - 1;
				while (l <= r) {
					m = (l + r) / 2;
					if (sum[m] - sum[k[1]] < sum[k[0]] - M || sum[m] - sum[k[1]] < sum[k[1]] - sum[k[0]] - M || sum[m] - sum[k[1]] < sum[N] - sum[m] - M || sum[k[0]] < sum[N] - sum[m] - M || sum[k[1]] - sum[k[0]] < sum[N] - sum[m] - M) l = m + 1;
					else if (sum[m] - sum[k[1]] > sum[k[0]] + M || sum[m] - sum[k[1]] > sum[k[1]] - sum[k[0]] + M || sum[m] - sum[k[1]] > sum[N] - sum[m] + M || sum[k[0]] > sum[N] - sum[m] + M || sum[k[1]] - sum[k[0]] > sum[N] - sum[m] + M) r = m - 1;
					else break;
				}
				if (l <= r) break;
			}
			if (k[1] <= N - 2 && sum[k[1]] - sum[k[0]] <= (sum[N] - sum[k[1]]) / 2 + M && sum[k[0]] <= (sum[N] - sum[k[1]]) / 2 + M && sum[k[1]] - sum[k[0]] <= sum[k[0]] + M) break;
		}
		if (k[0] <= N - 3 && sum[k[0]] <= (sum[N] - sum[k[0]]) / 3 + M) R = M;
		else L = M + 1;
	}
	printf("%lld\n", L);
	fflush(stdout);
	return 0;
}