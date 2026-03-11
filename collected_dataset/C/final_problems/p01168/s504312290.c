// AOJ 2044: Lying about Your Age
// 2017.11.04 bal4u@uu

#include <stdio.h>
#include <string.h>
#define TM 0
#if TM
#include <time.h>
#endif

#define INF 11001001
int tbl[201][17];
char buf[20], *bp;

int getint()
{
	int n = 0;
	while (*bp >= '0') n = (n<<3) + (n<<1) + (*bp++ & 0xf);
	return n;
}

int cmp(int *a, int *b) { return *b - *a; }

int main()
{
	int a, b, c, i, j, t, ans;

#if TM
	clock_t start, end;
	start = clock();
#endif
	for (j = 1; j <= 200; j++) for (i = 2; i <= 16; i++) {
		for (t = j, a = 0, c = 1; t; t /= i) {
			if ((b = t % i) > 9) { a = INF; break; }
			a += c*b, c *= 10;
		}
		tbl[j][i] = a;
	}

	while (scanf("%d%d%d", &a, &b, &c) && a >= 0) {
		ans = INF;
		for (i = 2; i <= 16; i++) if (tbl[a][i] == b) { ans = b; break; }
		if (ans == b) for (j = a+1; j <= c; j++) {
			for (t = INF, i = 2; i <= 16; i++) if (tbl[j][i] >= ans) {
				if (tbl[j][i] < t) t = tbl[j][i];
			}
			if (t == INF) break;
			ans = t;
		}
		if (ans < INF) printf("%d\n", ans);
		else puts("-1");
	}
#if TM
	end = clock();
	printf("time %lf\n", (double)(end-start)/CLOCKS_PER_SEC);
#endif
	return 0;
}