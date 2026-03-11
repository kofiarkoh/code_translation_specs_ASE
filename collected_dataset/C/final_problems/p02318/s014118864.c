#include <stdio.h>          // printf(), scanf()
#include <string.h>         // strlen()

#define L_MAX 1000
#define min(a, b) ((a) < (b) ? (a) : (b))

int
main(int argc, char** argv)
{
	char s[L_MAX + 1];
	char t[L_MAX + 1];
	int dp[L_MAX + 1][L_MAX + 1];
	int i, j;

	scanf("%s", s);
	scanf("%s", t);

	int u = strlen(s);
	int v = strlen(t);
	for (i = 0; i <= u; ++i)
		dp[i][0] = i;

	for (j = 1; j <= v; ++j)
		dp[0][j] = j;

	for (i = 1; i <= u; ++i)
	{
		for (j = 1; j <= v; ++j)
		{
			int cost = (s[i - 1] == t[j - 1]) ? 0 : 1;
			dp[i][j] = min(dp[i - 1][j] + 1, dp[i][j - 1] + 1);
			dp[i][j] = min(dp[i][j], dp[i - 1][j - 1] + cost);
		}
	}

	printf("%d\n", dp[u][v]);
	return 0;
}