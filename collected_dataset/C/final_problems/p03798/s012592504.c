#include<stdio.h>
int main()
{
	int n;
	scanf("%d", &n);
	char s[100005];
	scanf("%s", s);
	int i, j;
	char ans[4][100005];
	ans[0][0] = ans[1][0] = 'S';
	ans[2][0] = ans[3][0] = 'W';
	ans[0][1] = ans[2][1] = 'S';
	ans[1][1] = ans[3][1] = 'W';
	for (i = 1; i < n - 1; i++)
	{
		for (j = 0; j < 4; j++)
		{
			if (ans[j][i - 1] == 'S' && ans[j][i] == 'S' && s[i] == 'o')ans[j][i + 1] = 'S';
			if (ans[j][i - 1] == 'S' && ans[j][i] == 'S' && s[i] == 'x')ans[j][i + 1] = 'W';
			if (ans[j][i - 1] == 'S' && ans[j][i] == 'W' && s[i] == 'o')ans[j][i + 1] = 'W';
			if (ans[j][i - 1] == 'S' && ans[j][i] == 'W' && s[i] == 'x')ans[j][i + 1] = 'S';
			if (ans[j][i - 1] == 'W' && ans[j][i] == 'S' && s[i] == 'o')ans[j][i + 1] = 'W';
			if (ans[j][i - 1] == 'W' && ans[j][i] == 'S' && s[i] == 'x')ans[j][i + 1] = 'S';
			if (ans[j][i - 1] == 'W' && ans[j][i] == 'W' && s[i] == 'o')ans[j][i + 1] = 'S';
			if (ans[j][i - 1] == 'W' && ans[j][i] == 'W' && s[i] == 'x')ans[j][i + 1] = 'W';
		}
	}
	for (i = 0; i < 4; i++)
	{
		if (ans[i][0] == 'S' && ans[i][1] == 'S' && ans[i][n - 1] == 'S' && s[0] == 'x')continue;
		if (ans[i][0] == 'S' && ans[i][1] == 'S' && ans[i][n - 1] == 'W' && s[0] == 'o')continue;
		if (ans[i][0] == 'S' && ans[i][1] == 'W' && ans[i][n - 1] == 'S' && s[0] == 'o')continue;
		if (ans[i][0] == 'S' && ans[i][1] == 'W' && ans[i][n - 1] == 'W' && s[0] == 'x')continue;
		if (ans[i][0] == 'W' && ans[i][1] == 'S' && ans[i][n - 1] == 'S' && s[0] == 'o')continue;
		if (ans[i][0] == 'W' && ans[i][1] == 'S' && ans[i][n - 1] == 'W' && s[0] == 'x')continue;
		if (ans[i][0] == 'W' && ans[i][1] == 'W' && ans[i][n - 1] == 'S' && s[0] == 'x')continue;
		if (ans[i][0] == 'W' && ans[i][1] == 'W' && ans[i][n - 1] == 'W' && s[0] == 'o')continue;
		if (ans[i][n - 1] == 'S' && ans[i][n - 2] == 'S' && ans[i][0] == 'S' && s[n - 1] == 'x')continue;
		if (ans[i][n - 1] == 'S' && ans[i][n - 2] == 'S' && ans[i][0] == 'W' && s[n - 1] == 'o')continue;
		if (ans[i][n - 1] == 'S' && ans[i][n - 2] == 'W' && ans[i][0] == 'S' && s[n - 1] == 'o')continue;
		if (ans[i][n - 1] == 'S' && ans[i][n - 2] == 'W' && ans[i][0] == 'W' && s[n - 1] == 'x')continue;
		if (ans[i][n - 1] == 'W' && ans[i][n - 2] == 'S' && ans[i][0] == 'S' && s[n - 1] == 'o')continue;
		if (ans[i][n - 1] == 'W' && ans[i][n - 2] == 'S' && ans[i][0] == 'W' && s[n - 1] == 'x')continue;
		if (ans[i][n - 1] == 'W' && ans[i][n - 2] == 'W' && ans[i][0] == 'S' && s[n - 1] == 'x')continue;
		if (ans[i][n - 1] == 'W' && ans[i][n - 2] == 'W' && ans[i][0] == 'W' && s[n - 1] == 'o')continue;
		ans[i][n] = '\0';
		printf("%s\n", ans[i]);
		return 0;
	}
	printf("-1\n");
	return 0;
}