// Aizu 2186: Heian-Kyo Walking
// 2017.9.20 bal4u@uu

#include <stdio.h>
#include <string.h>

int dp[17][17];
char map[17][17][17][17];

int main()
{
	int t, n, h, w, r, c, r2, c2, x;

	scanf("%d", &t);
	while (t-- > 0) {
		scanf("%d%d%d", &w, &h, &n);
		memset(map, 0, sizeof(map)), memset(dp, 0, sizeof(dp));
		while (n-- > 0) {
			scanf("%d%d%d%d", &c, &r, &c2, &r2);
			if (c > c2) x = c, c = c2, c2 = x;
			if (r > r2) x = r, r = r2, r2 = x;
			map[r][c][r2][c2] = 1;
		}
		dp[0][0] = 1;
		for (r = 0; r <= h; r++) for (c = 0; c <= w; c++) {
			if (!map[r][c][r+1][c]) dp[r+1][c] += dp[r][c];
			if (!map[r][c][r][c+1]) dp[r][c+1] += dp[r][c];
		}
		if (!dp[h][w]) puts("Miserable Hokusai!");
		else printf("%d\n", dp[h][w]);
	}
	return 0;
}