#include <stdio.h>
#include <string.h>

#define N	25000
#define K	400
#define MD	1000000007

int max(int a, int b) { return a > b ? a : b; }

int vv[K + 1], ff[K + 1], gg[K + 1];

void init() {
	int i;

	ff[0] = gg[0] = 1;
	for (i = 1; i <= K; i++) {
		vv[i] = i == 1 ? 1 : (long long) vv[i - MD % i] * (MD / i + 1) % MD;
		ff[i] = (long long) ff[i - 1] * i % MD;
		gg[i] = (long long) gg[i - 1] * vv[i] % MD;
	}
}

int main() {
	static int aa[N], pp[K], dp[N + 1][K], dq[N + 1][K];
	int n, k, m, i, j, ans;

	init();
	scanf("%d%d%d", &n, &k, &m);
	for (i = 0; i < m; i++)
		scanf("%d", &aa[i]), aa[i]--;
	memset(pp, -1, k * sizeof *pp);
	for (i = -1, j = 0; j < m; j++) {
		i = max(i, pp[aa[j]]), pp[aa[j]] = j;
		if (j - i == k)
			break;
	}
	ans = n - m + 1;
	for (i = 0; i < n - m; i++)
		ans = (long long) ans * k % MD;
	if (j < m) {
		printf("%d\n", ans);
		return 0;
	}
	memset(pp, -1, k * sizeof *pp);
	j = 0;
	while (j < m && pp[aa[j]] == -1)
		pp[aa[j]] = j, j++;
	if (j < m) {
		dp[0][j] = 1;
		for (i = 1; i <= n - m; i++) {
			for (j = k - 1; j > 0; j--)
				dp[i][j] = ((j + 1 == k ? 0 : dp[i][j + 1]) + dp[i - 1][j]) % MD;
			for (j = 1; j < k; j++)
				dp[i][j] = (dp[i][j] + (long long) dp[i - 1][j - 1] * (k - j + 1)) % MD;
		}
		memset(pp, -1, k * sizeof *pp);
		j = m - 1;
		while (j >= 0 && pp[aa[j]] == -1)
			pp[aa[j]] = j, j--;
		dq[0][m - 1 - j] = 1;
		for (i = 1; i <= n - m; i++) {
			for (j = k - 1; j > 0; j--)
				dq[i][j] = ((j + 1 == k ? 0 : dq[i][j + 1]) + dq[i - 1][j]) % MD;
			for (j = 1; j < k; j++)
				dq[i][j] = (dq[i][j] + (long long) dq[i - 1][j - 1] * (k - j + 1)) % MD;
		}
		for (i = 0; i <= n - m; i++) {
			int x, y;

			x = 0, y = 0;
			for (j = 1; j < k; j++) {
				x = (x + dp[i][j]) % MD;
				y = (y + dq[n - m - i][j]) % MD;
			}
			ans = (ans - (long long) x * y) % MD;
		}
	} else {
		dp[0][0] = 1, dq[0][0] = 0;
		for (i = 1; i <= n; i++) {
			for (j = k - 1; j > 0; j--) {
				dp[i][j] = ((j + 1 == k ? 0 : dp[i][j + 1]) + dp[i - 1][j]) % MD;
				dq[i][j] = ((j + 1 == k ? 0 : dq[i][j + 1]) + dq[i - 1][j]) % MD;
			}
			for (j = 1; j < k; j++) {
				dp[i][j] = (dp[i][j] + (long long) dp[i - 1][j - 1] * (k - j + 1)) % MD;
				dq[i][j] = (dq[i][j] + (long long) dq[i - 1][j - 1] * (k - j + 1)) % MD;
			}
			for (j = m; j < k; j++)
				dq[i][j] = (dq[i][j] + (long long) dp[i][j] * gg[k] % MD * ff[k - m]) % MD;
		}
		for (j = 1; j < k; j++)
			ans = (ans - dq[n][j]) % MD;
	}
	if (ans < 0)
		ans += MD;
	printf("%d\n", ans);
	return 0;
}
