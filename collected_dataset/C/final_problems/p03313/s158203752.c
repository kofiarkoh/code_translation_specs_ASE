#include <stdio.h>

const int bit[19] = {1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144};

void update_BIT(int N, int BIT[], int i, int k)
{
	while (i <= N) {
		if (k > BIT[i]) BIT[i] = k;
		i += (i & -i);
	}
}

int max_BIT(int BIT[], int r)
{
	int max = 0;
	while (r > 0) {
		if (BIT[r] > max) max = BIT[r];
		r -= (r & -r);
	}
	return max;
}

int main()
{
	int i, N, A[262145];
	scanf("%d", &N);
	for (i = 0; i < bit[N]; i++) scanf("%d", &(A[i]));
	A[bit[N]] = -1;
	
	int j, k, dp[262144][2] = {}, BIT[262144] = {};
	for (i = 1, dp[0][0] = 0, dp[0][1] = bit[N]; i < bit[N]; i++) {
		dp[i][0] = i;
		for (j = i, k = 0; j > 0; j >>= 1, k++) {
			if ((j & 1) == 0) continue;
			if (A[dp[i^bit[k]][0]] > A[dp[i][0]]) {
				if (A[dp[i^bit[k]][1]] > A[dp[i][0]]) dp[i][1] = dp[i^bit[k]][1];
				else dp[i][1] = dp[i][0];
				dp[i][0] = dp[i^bit[k]][0];
			} else if (A[dp[i^bit[k]][0]] > A[dp[i][1]] && dp[i^bit[k]][0] != dp[i][0]) dp[i][1] = dp[i^bit[k]][0];
		}
		update_BIT(bit[N] - 1, BIT, i, A[dp[i][0]] + A[dp[i][1]]);
	}
	for (i = 1; i < bit[N]; i++) printf("%d\n", max_BIT(BIT, i));
	fflush(stdout);
	return 0;
}