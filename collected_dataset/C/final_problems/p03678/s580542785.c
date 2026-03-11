#include <stdio.h>
#include <string.h>

#define N	200000
#define A	26
#define K	100

int max(int a, int b) { return a > b ? a : b; }

int period(char *cc, int n) {
	static int zz[N];
	int i, l, r;

	for (i = 1, l = r = 0; i < n; i++)
		if (zz[i - l] < r - i)
			zz[i] = zz[i - l];
		else {
			l = i, r = max(r, l);
			while (r < n && cc[r] == cc[r - l])
				r++;
			zz[i] = r - l;
		}
	for (i = 1; i < n; i++)
		if (zz[i] == n - i)
			break;
	return i;
}

long long ans[A];

void solve(char *cc, int n, int m, long long r, int sign) {
	static long long len[K], cnt[K][A];
	int h, i, a, k;

	len[0] = m, memset(cnt[0], 0, sizeof cnt[0]);
	for (i = 0; i < m; i++)
		cnt[0][cc[i] - 'a']++;
	len[1] = n, memset(cnt[1], 0, sizeof cnt[1]);
	for (i = 0; i < n; i++)
		cnt[1][cc[i] - 'a']++;
	k = 2;
	while (len[k - 1] < r) {
		len[k] = len[k - 1] + len[k - 2];
		for (a = 0; a < A; a++)
			cnt[k][a] = cnt[k - 1][a] + cnt[k - 2][a];
		k++;
	}
	for (h = k - 1; h >= 0; h--)
		if (r >= len[h]) {
			for (a = 0; a < A; a++)
				ans[a] += cnt[h][a] * sign;
			r -= len[h];
		}
	for (i = 0; i < r; i++)
		ans[cc[i] - 'a'] += sign;
}

int main() {
	static char cc[N + 1];
	int n, m, i, a;
	long long l, r;

	scanf("%s%lld%lld", cc, &l, &r), n = strlen(cc) / 2, l--;
	m = period(cc, n);
	if (n % m == 0) {
		while (l < r && l % m != 0)
			ans[cc[l++ % m] - 'a']++;
		while (l < r && r % m != 0)
			ans[cc[--r % m] - 'a']++;
		for (i = 0; i < m; i++)
			ans[cc[i] - 'a'] += (r - l) / m;
	} else
		solve(cc, n, m, r, 1), solve(cc, n, m, l, -1);
	for (a = 0; a < A; a++)
		printf("%lld ", ans[a]);
	printf("\n");
	return 0;
}
