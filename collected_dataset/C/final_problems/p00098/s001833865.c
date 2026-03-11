// AOJ 0098: Maximum Sum Sequence II 
// 2017.8.15 bal4u@uu
// 2017.11.1 retry

#include <stdio.h>

int a[102][102];
int s[102][102];

char buf[1000], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int main()
{
	int n, r, c, k, ans;

	fgets(p=buf, 10, stdin), n = getint();
	for (r = 0; r < n; r++) {
		fgets(p=buf, sizeof(buf), stdin);
		for (c = 0; c < n; c++, p++) {
			if (*p == '-') p++, a[r][c] = -getint();
			else                a[r][c] =  getint();
		}
	}

	for (r = 0; r < n; r++)
		for (c = 0; c < n; c++)
			s[r][c+1] += s[r][c] + a[r][c];

	ans = s[0][1];
	for (c = 0; c < n; c++) for (k = c+1; k <= n; k++) {
		int t = 0;
		for (r = 0; r < n; r++) {
			if (t < 0) t  = s[r][k] - s[r][c];
			else       t += s[r][k] - s[r][c];
			if (t > ans) ans = t;
		}
	}
	printf("%d\n", ans);
	return 0;
}