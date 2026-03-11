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
		t = j, a = 0;
		while (t) {
			b = t % i;
			if (b > 9) { a = INF; break; }
			buf[a++] = b;
			t /= i;
		}
		if (a < INF) {
			b = 0; while (--a >= 0) b = 10*b + buf[a];
		} else b = a;
		tbl[j][i] = b;
	}

	while (scanf("%d%d%d", &a, &b, &c) && a >= 0) {
		ans = INF;
		for (i = 2; i <= 16; i++) if (tbl[a][i] == b) ans = b;
		for (j = a+1; j <= c; j++) {
			for (t = INF, i = 2; i <= 16; i++) if (ans <= tbl[j][i]) {
				if (tbl[j][i] < t) t = tbl[j][i];
			}
			ans = t;
		}
		if (ans == INF) puts("-1");
		else printf("%d\n", ans);
	}
#if TM
	end = clock();
	printf("time %lf\n", (double)(end-start)/CLOCKS_PER_SEC);
#endif
	return 0;
}