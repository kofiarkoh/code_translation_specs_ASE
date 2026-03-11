#include <stdio.h>
#include <sys/time.h>

#define N	50
#define S	(N * N * N * 2)
#define INF	0x3f3f3f3f
#define INF_	0x3f3f3f3f3f3f3f3fLL

int min(int a, int b) { return a < b ? a : b; }

unsigned int X;

void srand_() {
	struct timeval tv;

	gettimeofday(&tv, NULL);
	X = tv.tv_sec ^ tv.tv_usec;
}

int rand_() {
	return (X *= 3) >> 1;
}

long long xx[N]; int yy[N];

void sort(int *ii, int l, int r) {
	while (l < r) {
		int i = l, j = l, k = r, i_ = ii[l + rand_() % (r - l)], tmp;

		while (j < k) {
			long long c = xx[ii[j]] * yy[i_] - xx[i_] * yy[ii[j]];

			if (c == 0)
				j++;
			else if (c < 0) {
				tmp = ii[i], ii[i] = ii[j], ii[j] = tmp;
				i++, j++;
			} else {
				k--;
				tmp = ii[j], ii[j] = ii[k], ii[k] = tmp;
			}
		}
		sort(ii, l, i);
		l = k;
	}
}

int main() {
	static int pp[N], ii[N], kk[N], qu[S + 1];
	static long long dp[S + 1], dq[S + 1], tt[S + 1];
	int n, x, k_, i, r, s, head, cnt, ans, y;

	srand_();
	scanf("%d%d%d", &n, &x, &k_);
	for (i = 0; i < n; i++) {
		scanf("%lld", &xx[i]);
		yy[i] = 1;
		if (i > 0)
			scanf("%d", &pp[i]), pp[i]--;
	}
	for (i = n - 1; i > 0; i--) {
		xx[pp[i]] += xx[i];
		yy[pp[i]] += yy[i];
	}
	for (i = 0; i < n; i++)
		ii[i] = i;
	sort(ii, 0, n);
	ans = 0;
	for (i = 0; i < n; i++) {
		kk[ii[i]] = min(x / xx[ii[i]], ii[i] == 0 ? INF : k_);
		x -= xx[ii[i]] * kk[ii[i]];
		ans += yy[ii[i]] * kk[ii[i]];
		if (kk[ii[i]] < k_)
			break;
	}
	for (s = 0; s <= S; s++)
		dp[s] = INF_;
	dp[0] = 0;
	for (i = 0; i < n; i++) {
		long long x = xx[i];
		int k;

		y = yy[i], k = i == 0 ? INF : k_ - kk[i];
		if (k > 0) {
			for (r = 0; r < y; r++) {
				head = cnt = 0;
				for (s = r; s <= S; s += y) {
					if (dp[s] != INF_) {
						while (cnt && dp[s] - (s / y) * x >= dp[qu[head + cnt - 1]] - (qu[head + cnt - 1] / y) * x)
							cnt--;
						qu[head + cnt++] = s;
					}
					tt[s] = cnt == 0 ? INF_ : dp[qu[head]] + (s - qu[head]) / y * x;
					if (cnt && s / y >= k && qu[head] == s - k * y)
						head++, cnt--;
				}
			}
			for (s = 0; s <= S; s++)
				dp[s] = tt[s];
		}
	}
	for (s = 0; s <= S; s++)
		dq[s] = -INF_;
	dq[0] = 0;
	for (i = 0; i < n; i++) {
		long long x = xx[i];
		int k;

		y = yy[i], k = kk[i];
		if (k > 0) {
			for (r = 0; r < y; r++) {
				head = cnt = 0;
				for (s = r; s <= S; s += y) {
					if (dq[s] != -INF_) {
						while (cnt && dq[s] - (s / y) * x <= dq[qu[head + cnt - 1]] - (qu[head + cnt - 1] / y) * x)
							cnt--;
						qu[head + cnt++] = s;
					}
					tt[s] = cnt == 0 ? -INF_ : dq[qu[head]] + (s - qu[head]) / y * x;
					if (cnt && s / y >= k && qu[head] == s - k * y)
						head++, cnt--;
				}
			}
			for (s = 0; s <= S; s++)
				dq[s] = tt[s];
		}
	}
	for (y = n * 2; y >= 0; y--)
		for (s = y; s <= S; s++)
			if (x >= dp[s] - dq[s - y]) {
				ans += y;
				printf("%d\n", ans);
				return 0;
			}
	return 0;
}
