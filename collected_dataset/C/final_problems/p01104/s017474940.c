// AOJ 1619 Making Lunch Boxes
// 2018.2.12 bal4u

#include <stdio.h>
#include <string.h>

char b[503][503];
char d[503];
short dp[2][1 << 21];
int a[1 << 24];

#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int bitcount(int bits)
{
    bits = (bits & 0x55555555) + (bits >> 1 & 0x55555555);
    bits = (bits & 0x33333333) + (bits >> 2 & 0x33333333);
    bits = (bits & 0x0f0f0f0f) + (bits >> 4 & 0x0f0f0f0f);
    bits = (bits & 0x00ff00ff) + (bits >> 8 & 0x00ff00ff);
    return (bits & 0x0000ffff) + (bits >>16 & 0x0000ffff);
}

int main()
{
	int n, m, i, j, k, f, cnt, lim, ans;
	int k1, k2;
	char *p;

	while (n = in()) {
		m = in();
		for (i = 0; i < n; i++) {
			fgets(p=b[i], 503, stdin);
			for (j = 0; j < m; j++) *p &= 1, p++;
		}

		ans = 0;
		if (m > 30) {
			lim = 1 << n;
			for (i = 0; i < lim; i++) {
				cnt = 0;
				memset(d, 0, m);
				for (j = 0; j < n; j++) if ((i >> j) & 1) {
					cnt++;
					p = b[j]; for (k = 0; k < m; k++) d[k] ^= *p++;
				}
				f = 1; for (k = 0; k < m; k++) if (d[k]) { f = 0; break; }
				if (f && ans < cnt) ans = cnt;
			}
		} else if (n <= 24) {
			lim = 1 << n;
			a[0] = 0;
			for (j = 0; j < n; j++) {
				f = 0;
				p = b[j], k = m; while (k--) f = (f << 1) | *p++;
				a[1 << j] = f;
			}
			for (j = 1; j < lim; j++) {
				f = j & -j;
				if (j > f) a[j] = a[f] ^ a[j-f];
			}
			for (j = 0; j < lim; j++) if (!a[j]) {
				f = bitcount(j);
				if (ans < f) ans = f;
			}
		} else {
			lim = 1 << m;
			memset(dp, -1, lim << 2);
			dp[0][0] = 0;
			for (k1 = 0, k2 = 1, j = 0; j < n; j++, k1 = k2, k2 = !k2) {
				f = 0;
				p = b[j], k = m; while (k--) f = (f << 1) | *p++;
				memcpy(dp[k2], dp[k1], lim << 2);
				for (k = 0; k < lim; k++) if (dp[k1][k] >= 0) {
					int x = k ^ f;
					if (dp[k2][x] < dp[k1][k] + 1) dp[k2][x] = dp[k1][k] + 1;
				}
			}
			ans = dp[k1][0];
		}
		printf("%d\n", ans);
	}
	return 0;
}

