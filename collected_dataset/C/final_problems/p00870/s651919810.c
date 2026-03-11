// AOJ 1291 Search of Concatenated Strings
// 2018.5.6 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

char e[12][22]; int w[12];
char t[5001]; int len;
char dp[5001][1<<12];
  
int main()
{
	int n, m, i, j, k, S, lim, ans;
	char *p;

	while (n = in()) {
		m = in();

		for (i = 0; i < n; i++) {
			p = e[i]; while ((*p = gc()) > ' ') p++;
			*p = 0, w[i] = p-e[i];
		}

		p = t; while (m--) {
			while ((*p = gc()) > ' ') p++;
		}
		*p = 0, len = p-t;

		lim = (1<<n)-1;
		memset(dp, 0, sizeof(dp));

	    for (i = 0; i < len; i++) {
			for (j = 0; j < n; j++) if ((k = i+1-w[j]) >= 0) {
				if (memcmp(t+k, e[j], w[j])) continue;
  
				for (S = 0; S <= lim; S++) {
					if ((S >> j) & 1) continue;
					dp[i+1][S|(1<<j)] |= dp[k][S];
				}
				dp[i+1][1<<j] = 1;
			}
		}
  
		ans = 0;
		for (i = 0; i <= len; i++) if (dp[i][lim]) ans++;
		printf("%d\n", ans);
	}
	return 0;
}
