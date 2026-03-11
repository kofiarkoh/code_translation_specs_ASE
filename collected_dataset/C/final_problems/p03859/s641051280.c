#include <stdio.h>
#include <string.h>

#define N	3000
#define MD	1000000007

int max(int a, int b) { return a > b ? a : b; }

int main() {
	static char cc[N + 1];
	static int dp[N + 1];
	int n, m, k, k0, l_, r_;

	scanf("%d%d%s", &n, &m, cc);
	l_ = 0, r_ = -1, k0 = 0;
	dp[0] = 1;
	while (m--) {
		int l, r;

		scanf("%d%d", &l, &r), l--, r--;
		if (r_ < r) {
			while (l_ <= r_ && l_ < l) {
				for (k = k0; k > 0; k--)
					dp[k] = (dp[k] + dp[k - 1]) % MD;
				if (k0 >= r_ - l_ + 1)
					dp[k0 - (r_ - l_ + 1)] = 0;
				l_++;
			}
			if (r_ < l)
				l_ = l, r_ = l - 1;
			while (r_ < r)
				if (cc[++r_] == '0')
					k0++;
		}
	}
	while (l_ <= r_) {
		for (k = k0; k > 0; k--)
			dp[k] = (dp[k] + dp[k - 1]) % MD;
		if (k0 >= r_ - l_ + 1)
			dp[k0 - (r_ - l_ + 1)] = 0;
		l_++;
	}
	printf("%d\n", dp[k0]);
	return 0;
}
