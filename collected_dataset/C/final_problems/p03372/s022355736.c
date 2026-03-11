#include <stdio.h>

int main()
{
	int i, N;
	long long C, x[100002], v[100002];
	scanf("%d %lld", &N, &C);
	for (i = 1; i <= N; i++) scanf("%lld %lld", &(x[i]), &(v[i]));
	x[0] = 0;
	x[N+1] = C;
	
	long long ans = 0, sum[2][100002] = {}, max[2][100002] = {};
	for (i = 1; i <= N; i++) {
		sum[0][i] = sum[0][i-1] + v[i] - (x[i] - x[i-1]);
		max[0][i] = (sum[0][i] > max[0][i-1])? sum[0][i]: max[0][i-1];
		if (sum[0][i] > ans) ans = sum[0][i];
	}
	for (i = N; i >= 1; i--) {
		sum[1][i] = sum[1][i+1] + v[i] - (x[i+1] - x[i]);
		max[1][i] = (sum[1][i] > max[1][i+1])? sum[1][i]: max[1][i+1];
		if (sum[1][i] > ans) ans = sum[1][i];
	}
	for (i = 1; i <= N; i++) {
		if (sum[0][i] - x[i] + max[1][i+1] > ans) ans = sum[0][i] - x[i] + max[1][i+1];
		if (sum[1][i] - (C - x[i]) + max[0][i-1] > ans) ans = sum[1][i] - (C - x[i]) + max[0][i-1];
	}
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}