// Aizu Vol-0 0097: Sum of Integers II
// 2017.9.1 bal4u@uu

#include <stdio.h>

int p[10][101][902];

int main()
{
	int n, s;
	int i, j, k, min, max, ans;

	for (i = 0; i <= 100; i++) p[1][i][i] = 1;
	for (min = 0, max = 100, n = 1; n < 9; n++, min++, max+=100)
		for (i = 0; i <= 100; i++)
			for (j = min; j <= max; j++)
				for (k = 1; k + i <= 100; k++) p[n+1][i+k][j+i+k] += p[n][i][j];

	while (scanf("%d%d", &n, &s) && n) {
		if (s > 864) puts("0");
		else {
			for (ans = i = 0; i <= 100; i++) ans += p[n][i][s];
			printf("%d\n", ans);
		}
	}
	return 0;
}