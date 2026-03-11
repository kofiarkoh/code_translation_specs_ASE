#include <stdio.h>

int main()
{
	int i, N, K;
	long long a[100001];
	scanf("%d %d", &N, &K);
	for (i = 1; i <= N; i++) scanf("%lld", &(a[i]));
	
	long long sum[2][100001], max = 0, tmp;
	for (i = 1, sum[0][0] = 0, sum[1][0] = 0; i <= N; i++) {
		sum[0][i] = sum[0][i-1] + a[i];
		sum[1][i] = sum[1][i-1] + ((a[i] > 0)? a[i]: 0);
	}
	for (i = 1; i <= N - K + 1; i++) {
		tmp = (sum[0][i+K-1] > sum[0][i-1])? sum[0][i+K-1] - sum[0][i-1]: 0;
		if (sum[1][i-1] + sum[1][N] - sum[1][i+K-1] + tmp > max) max = sum[1][i-1] + sum[1][N] - sum[1][i+K-1] + tmp;
	}
	
	printf("%lld\n", max);
	fflush(stdout);
	return 0;
}