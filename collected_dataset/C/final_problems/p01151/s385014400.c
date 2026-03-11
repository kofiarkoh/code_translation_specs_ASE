// AOJ 2026: Divisor is the Conqueror
// 2017.11.03 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <time.h>

#define HASHSIZ 100019
long long hash[HASHSIZ+2], *hashend = hash + HASHSIZ;

int insert(long long n)
{
	long long *p = hash + n % HASHSIZ;
	while (*p) {
		if (*p == n) return 1;
		if (++p == hashend) p = hash;
	}
	*p = n;
	return 0;
}

int c[15];
int ans[54];
char f[14][365];		// 364=(13+1)*13/2*4

int rec(int j, int k)
{
	int i;
	long long s;

	if (!j) return 1;
	for (s = 0, i = 1; i <= 13; i++) s = (s<<2)+s + c[i];
	if (insert(s)) return 0;

	for (i = 1; i <= 13; i++) {
		if (!c[i]) continue;
		if (f[i][k-i]) {
			c[i]--, ans[j] = i;
			if (rec(j-1, k-i)) return 1;
			c[i]++;
		}
	}
	return 0;
}

int main()
{
	int n, i, v, s;
//	clock_t start, end;

//	start = clock();
	memset(f[1], 1, sizeof(f[1]));
	for (i = 2; i <= 13; i++) {
		f[i][0] = 1;
		for (v = i; v < 365; v += i) f[i][v] = 1;
	}

	while (scanf("%d", &n) && n > 0) {
		if (n == 1) { scanf("%d", &v); printf("%d\n", v); continue; }

		memset(c, 0, sizeof(c));
		for (s = 0, i = 0; i < n; i++) {
			scanf("%d", &v);
			s += v, c[v]++;
		}
		memset(hash, 0, sizeof(hash));
		if (rec(n, s)) {
			printf("%d", ans[1]);
			for (i = 2; i <= n; i++) printf(" %d", ans[i]);
			putchar('\n');
		} else puts("No");
	}
//	end = clock();
//	printf("time %lf\n", (double)(end-start)/CLOCKS_PER_SEC);
	return 0;
}