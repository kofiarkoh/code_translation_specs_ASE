#include <stdio.h>
int main() {
	int n, m, p, ans = 0;
	char s[110][11] = {}, t[110][11] = {};
	scanf("%d", &n);
	for (int i = 0; i < n; i++)
	{
		scanf("%s", s[i]);
	}
	scanf("%d", &m);
	for (int i = 0; i < m; i++)
	{
		scanf("%s", t[i]);
	}
	for (int i = 0; i < n; i++)
	{
		p = 0;
		for (int j = i; j < n; j++)
		{
			for (int k = 0; (s[i][k] == s[j][k]) && (k < 11); k++)
			{
				if (s[i][k] == '\0')
				{
					p++;
					break;
				}
			}
		}
		for (int j = 0; j < m; j++)
		{
			for (int k = 0; (s[i][k] == t[j][k]) && (k < 11); k++)
			{
				if (s[i][k] == '\0')
				{
					p--;
					break;
				}
			}
		}
		ans = (p > ans) ? p : ans;
    }
	printf("%d", ans);
	return 0;
}