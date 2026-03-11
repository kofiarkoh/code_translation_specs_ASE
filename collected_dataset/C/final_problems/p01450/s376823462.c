// AOJ 2333: My friends are small
// 2017.11.19 bal4u@uu

#include <stdio.h>

#define M 1000000007 
int w[202], s[202];
int f[10001];
int dp[10001];

char buf[20], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int main()
{
	int N, W, i, j, v, min, max, ans;

	fgets(p=buf, 20, stdin), N = getint(), p++, W = getint();
	fgets(p=buf, 10, stdin), min = max = v = getint(), f[v] = 1;
	i = N-1; while (i--) {
		fgets(p=buf, 10, stdin);
		v = getint(), f[v]++;
		if (v < min) min = v;
		else if (v > max) max = v;
	}
	for (j = 0, i = min; i <= max; i++) while (f[i]) w[j++] = i, f[i]--;
	s[0] = w[0]; for (i = 1; i < N; i++) s[i] = s[i-1] + w[i]; 

	if (s[N-1] <= W) { puts("1"); return 0; }
	dp[0] = 1, ans = 0;
	for (i = N-1; i >= 0; i--) {
		for (j = W-s[i]+1; j <= W-s[i]+w[i]; j++) {
			if (j >= 0) ans = (ans + dp[j]) % M;
		}
		for (j = W-w[i]; j >= 0; j--) dp[j+w[i]] = (dp[j+w[i]] + dp[j]) % M;
	}
	printf("%d\n", ans);
	return 0;
}