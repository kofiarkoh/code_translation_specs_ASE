// Aizu 2182: Eleven Lover
// 2017.9.20 bal4u@uu

#include <stdio.h>
#include <string.h>

char a[80005];
int dp[2][11];

int main()
{
	int i, d, k1, k2, ans;
	char *p;

	while (scanf("%s", a) && *a != '0') {
		memset(dp, 0, sizeof(dp)); ans = 0;
		for (k1 = 0, k2 = 1, p = a; *p; k1 = k2, k2 = !k2) {
			for (i = 0; i < 11; i++) dp[k2][i] = 0;
			d = *p++ - '0';
			for (i = 0; i < 11; i++) dp[k2][(10*i+d) % 11] += dp[k1][i]; 
			ans += dp[k2][0];
			if (d > 0) dp[k2][d]++;
		}
		printf("%d\n", ans);
	}
	return 0;
}