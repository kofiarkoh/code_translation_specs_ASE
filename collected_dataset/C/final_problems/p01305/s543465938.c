// AOJ 2187: Card Game
// 2017.11.6 bal4u@uu

#include <stdio.h>
#include <string.h>
#define TM 0
#if TM
#include <time.h>
#endif

#define P9	2.7557319223985890652557319223986e-6	// =1/9!
int g[10], j[10];
int f[10];
int ans;

void rec(int i, int sg, int sj)
{
	int k;
	if (i == 9) { ans += (sg > sj); return; }
	for (k = 0; k < 9; k++) {
		if (f[k]) continue;
		f[k] = 1;
		if (g[i] > j[k]) rec(i+1, sg+g[i]+j[k], sj);
		else             rec(i+1, sg, sj+g[i]+j[k]);
		f[k] = 0;
	}
}

int main()
{
	int t, i;
#if TM
	clock_t start, end;
	start = clock();
#endif
	scanf("%d", &t);
	while (t-- > 0) {
		for (i = 0; i < 9; i++) scanf("%d", g+i);
		for (i = 0; i < 9; i++) scanf("%d", j+i);
		memset(f, 0, sizeof(f));
		ans = 0;
		rec(0, 0, 0);
		printf("%.5lf %.5lf\n", ans*P9, 1-ans*P9);
	}
#if TM
	end = clock();
	printf("time %lf\n", (double)(end-start)/CLOCKS_PER_SEC);
#endif
	return 0;
}