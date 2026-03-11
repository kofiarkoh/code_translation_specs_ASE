#include <stdio.h>

int main()
{
	int i, N, h[200001];
	long long C;
	scanf("%d %lld", &N, &C);
	for (i = 1; i <= N; i++) scanf("%d", &(h[i]));
	
	int j, k = 1, l, r, m;
	long long dp[200001], q[200001][2];
	q[0][0] = -h[1] * 2;
	q[0][1] = (long long)h[1] * h[1];
	for (i = 2, dp[1] = 0; i <= N; i++) {
		l = 0;
		r = k - 1;
		while (l < r) {
			m = (l + r) / 2;
			if (q[m][0] * h[i] + q[m][1] < q[m+1][0] * h[i] + q[m+1][1]) r = m;
			else l = m + 1;
		}
		dp[i] = (long long)h[i] * h[i] + C + q[l][0] * h[i] + q[l][1];
		
		q[k][0] = -h[i] * 2;
		q[k][1] = dp[i] + (long long)h[i] * h[i];
		for (; k > 1; k--) {
			if ((q[k-1][0] - q[k][0]) * (q[k-1][1] - q[k-2][1]) >= (q[k-2][0] - q[k-1][0]) * (q[k][1] - q[k-1][1])) {
				q[k-1][0] = q[k][0];
				q[k-1][1] = q[k][1];
			} else break;
		}
		k++;
	}
	
	printf("%lld\n", dp[N]);
	fflush(stdout);
	return 0;
}