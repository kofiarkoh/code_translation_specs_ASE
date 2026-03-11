#include <stdio.h>
#include <string.h>

#define N 15

int main() {
	static int aa[N], xx[N][N - 1], yy[N][N - 1], honest[N];
	int n, i, j, b, k, yes, ans;

	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d", &aa[i]);
		for (j = 0; j < aa[i]; j++)
			scanf("%d%d", &xx[i][j], &yy[i][j]), xx[i][j]--;
	}
	ans = -1;
	for (b = 0; b < 1 << n; b++) {
		memset(honest, 0, n * sizeof *honest);
		for (i = 0; i < n; i++)
			if (b & 1 << i)
				honest[i] = 1;
		yes = 1;
		k = 0;
		for (i = 0; i < n; i++)
			if (honest[i]) {
				k++;
				for (j = 0; j < aa[i]; j++)
					if (honest[xx[i][j]] != yy[i][j])
						yes = 0;
			}
		if (yes && ans < k)
			ans = k;
	}
	printf("%d\n", ans);
	return 0;
}
