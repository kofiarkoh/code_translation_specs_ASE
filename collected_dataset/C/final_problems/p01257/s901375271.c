// AOJ 2138: Vending Machine
// 2017.11.10 bal4u@uu

#include <stdio.h>
#include <string.h>
#define TM 0
#if TM
#include <time.h>
#endif

int a[12], c[1025];
int dp[100002];

int cmp(int *a, int *b) { return *a - *b; }

int main()
{
	int n, m, i, j, sum, ans;
	int ii, k, s, lim;
#if TM
	clock_t start, end;
	start = clock();
#endif
	while (scanf("%d%d", &n, &m) && (n|m)) {
		for (i = 0; i < n; i++) scanf("%d", a+i);
		for (lim = 1 << n, j = 0; j < lim; j++) {
			for (s = 0, ii = 1, i = 0; i < n; i++, ii <<= 1) if (j & ii) s += a[i];
			c[j] = s;
		}
		sum = c[lim-1];
		if (m % sum <= lim-1 && c[m % sum] == 1) ans = m/sum + 1;
		else {
			memset(dp, 0x33, sizeof(dp)); dp[0] = 0;
		    for (j = 1; j < lim; j++) for (i = 0; (k=i+c[j]) <= m; i++) {
				if (dp[k] > dp[i]+1) dp[k] = dp[i]+1;
			}
			ans = dp[m];
		}
		printf("%d\n", ans);
	}
#if TM
	end = clock();
	printf("time %lf\n", (double)(end-start)/CLOCKS_PER_SEC);
#endif
	return 0;
}