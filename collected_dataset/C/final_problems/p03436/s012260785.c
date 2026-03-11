#include<stdio.h>
int h, w;
char s[55][55];
int v[55][55];
void dfs(int x, int y, int n)
{
	if (x < 0 || y < 0 || x >= h || y >= w)
		return;
	if (s[x][y] == '#')
		return;
	if (v[x][y] < n)
		return;
	v[x][y] = n;
	n++;
	dfs(x - 1, y, n);
	dfs(x, y - 1, n);
	dfs(x + 1, y, n);
	dfs(x, y + 1, n);
	return;
}
int main()
{
	scanf("%d %d", &h, &w);
	int i, j;
	for (i = 0; i < h; i++)
		scanf("%s", s[i]);
	for (i = 0; i < h; i++)
		for (j = 0; j < w; j++)
			v[i][j] = 1000000009;
	v[0][0] = 1;
	dfs(0, 1, 2);
	dfs(1, 0, 2);
	if (v[h - 1][w - 1] > 1000006)
	{
		printf("-1\n");
		return 0;
	}
	int count = 0;
	for (i = 0; i < h; i++)
		for (j = 0; j < w; j++)
			if (s[i][j] == '.')
				count++;
	printf("%d\n", count - v[h - 1][w - 1]);
	return 0;
}