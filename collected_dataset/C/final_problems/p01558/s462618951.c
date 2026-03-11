// AOJ 2444: Substring
// 2017.10.10 bal4u@uu

#include <stdio.h>
#include <ctype.h>

#define HASHSIZ 600011LL
unsigned long long hash[HASHSIZ+3], *hashend = hash + HASHSIZ;

#define M	1000000007
#define MAX 300000
char s[MAX+3];
unsigned long long hv[MAX+3], bv[MAX+3];

int insert(unsigned long long n)
{
	unsigned long long *p = hash + n % HASHSIZ;
	while (*p) {
		if (*p == n) return 0;
		if (++p == hashend) p = hash;
	}
	*p = n;
	return 1;
}

int main()
{
	int n, m, i, l, r, ans;
	char buf[10];

	scanf("%d%d%s", &n, &m, s);
	bv[0] = 1; for (i = 1; i <= n; i++) bv[i] = bv[i-1]*M;
	ans = 0, l = 0, r = 1, hv[0] = 0, hv[1] = s[0];
	while (m-- > 0) {
		scanf("%s", buf);
		if (*buf == 'R') {
			if (*(buf+1) == '+') {
				if (hv[++r] == 0) hv[r] = hv[r-1]*M + s[r-1];
			} else r--;
		} else {
			if (*(buf+1) == '+') l++; else l--;
		}
		ans += insert(hv[r] - hv[l] * bv[r-l]);
	}
	printf("%d\n", ans);
	return 0;
}