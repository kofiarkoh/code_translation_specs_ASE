// AOJ 2129: Text Justification
// 2017.11.9 bal4u@uu

#include <stdio.h>
#include <string.h>

int s[1002], dp[1002];

char buf[20], *bp;
int getint()
{
	int n = 0;
	while (*bp >= '0') n = (n<<3) + (n<<1) + (*bp++ & 0xf);
	return n;
}

int main()
{
	int n, w, i, j, t, ans;
	int cno = 0;

	while (1) {
		fgets(bp=buf, 20, stdin);
		n = getint(), bp++, w = getint();
		if (!n && !w) break;

		for (s[0] = 0, i = 1; i <= n; i++) {
			fgets(bp=buf, 10, stdin);
			s[i] = s[i-1] + getint();
		}
		memset(dp, 0x55, sizeof(dp)); dp[0] = 0;
		for (i = 1; i <= n; i++) for (j = 0; j < i; j++) {
			t = s[i] - s[j] - w;
			if (t < 0) t = -t;
			if (dp[j] + t < dp[i]) dp[i] = dp[j] + t;
		}
		for (ans = 0x7fffffff, i = 0; i < n; i++) {
			t = s[n] - s[i] - w;
			if (t > 0) dp[i] += t;
			if (ans > dp[i]) ans = dp[i];
		}
		printf("Case %d: %d\n", ++cno, ans);
	}
	return 0;
}