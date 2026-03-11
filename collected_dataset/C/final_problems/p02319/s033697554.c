#include <stdio.h>          // printf(), scanf()

#define MAX_N 100
#define MAX_V 100 
#define min(a, b) ((a) < (b) ? (a) : (b))

static const int INF = 2000000000;

int n, W;
int v[MAX_N], w[MAX_N];
int dp[MAX_N + 1][MAX_N * MAX_V + 1];

int
solve()
{
	int i, j;

	for (j = 0; j <= MAX_N * MAX_V; ++j)
		dp[0][j] = INF;

	dp[0][0] = 0;
	for (i = 0; i < n; ++i)
	{
		for (j = 0; j <= MAX_N * MAX_V; ++j)
		{
			if (j < v[i])
				dp[i + 1][j] = dp[i][j];
			else
				dp[i + 1][j] = min(dp[i][j], dp[i][j - v[i]] + w[i]);
		}
	}

	int res = 0;
	for (i = 0; i <= MAX_N * MAX_V; ++i)
	{
		if (dp[n][i] <= W)
			res = i;
	}

	return res;
}

int
main(int argc, char** argv)
{
	int i;

	scanf("%d %d", &n, &W);
	for (i = 0; i < n; ++i)
		scanf("%d %d", &v[i], &w[i]);

	printf("%d\n", solve());
	return 0;
}