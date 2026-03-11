#include<stdio.h>
int main()
{
	int h, w;
	scanf("%d %d", &h, &w);
	int i, j;
	char a[1003][1003];
	for (i = 0; i < h; i++)
		scanf("%s", a[i]);
	int c[1003][1003];
	for (i = 0; i < h; i++)
	{
		for (j = 0; j < w; j++)
		{
			if (a[i][j] == '#')
				c[i][j] = 0;
			else
				c[i][j] = 1000000009;
		}
	}
	for (i = 0; i < h; i++)
		for (j = 1; j < w; j++)
			if (c[i][j] > c[i][j - 1] + 1)
				c[i][j] = c[i][j - 1] + 1;
	for (i = 0; i < h; i++)
		for (j = w - 2; j >= 0; j--)
			if (c[i][j] > c[i][j + 1] + 1)
				c[i][j] = c[i][j + 1] + 1;
	for (j = 0; j < w; j++)
		for (i = 1; i < h; i++)
			if (c[i][j] > c[i - 1][j] + 1)
				c[i][j] = c[i - 1][j] + 1;
	for (j = 0; j < w; j++)
		for (i = h - 2; i >= 0; i--)
			if (c[i][j] > c[i + 1][j] + 1)
				c[i][j] = c[i + 1][j] + 1;
	int ans = 0;
	for (i = 0; i < h; i++)
		for (j = 0; j < w; j++)
			if (ans < c[i][j])
				ans = c[i][j];
	printf("%d\n", ans);
	return 0;
}