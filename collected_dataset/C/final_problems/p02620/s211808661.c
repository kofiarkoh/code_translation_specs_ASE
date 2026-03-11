#include<stdio.h>
int main()
{
	int d;
	scanf("%d", &d);
	int c[30];
	int i;
	for (i = 0; i < 26; i++)
		scanf("%d", &c[i]);
	int j;
	int s[400][30];
	for (i = 0; i < d; i++)
		for (j = 0; j < 26; j++)
			scanf("%d", &s[i][j]);
	int l[400][30];
	for (i = 0; i < 26; i++)
		l[0][i] = 0;
	int t[400];
	for (i = 0; i < d; i++)
	{
		scanf("%d", &t[i]);
		t[i]--;
	}
	int m;
	scanf("%d", &m);
	int dd, q;
	int a;
	for (i = 1; i <= d; i++)
	{
		for (j = 0; j < 26; j++)
			l[i][j] = l[i - 1][j];
		l[i][t[i - 1]] = i;
	}
	int ans = 0;
	for (i = 0; i < d; i++)
	{
		ans += s[i][t[i]];
		for (j = 0; j < 26; j++)
			ans -= c[j] * (i + 1 - l[i + 1][j]);
	}
	for (j = 0; j < m; j++)
	{
		scanf("%d %d", &dd, &q);
		dd--;
		q--;
		a = t[dd];
		ans -= s[dd][a];
		ans += s[dd][q];
		t[dd] = q;
		for (i = 0; i < d; i++)
		{
			ans += c[a] * (i + 1 - l[i + 1][a]);
			ans += c[q] * (i + 1 - l[i + 1][q]);
			l[i + 1][a] = l[i][a];
			l[i + 1][q] = l[i][q];
			l[i + 1][t[i]] = i + 1;
			ans -= c[a] * (i + 1 - l[i + 1][a]);
			ans -= c[q] * (i + 1 - l[i + 1][q]);
		}
		printf("%d\n", ans);
	}
	return 0;
}