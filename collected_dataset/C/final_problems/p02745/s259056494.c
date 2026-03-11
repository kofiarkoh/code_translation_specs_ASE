#include<stdio.h>
#include<string.h>
int min(int a, int b)
{
	if (a < b)
		return a;
	else
		return b;
}
int f(char s[], char t[], char r[])
{
	int ss, tt, rr;
	ss = strlen(s);
	tt = strlen(t);
	rr = strlen(r);
	int i, j, k;
	char a[10004];
	for (i = 0; i < 10004; i++)
		a[i] = '\0';
	for (i = 0; i < ss; i++)
		a[i] = s[i];
	for (i = 0; i <= ss; i++)
	{
		k = 0;
		for (j = 0; j < min(tt, ss - i); j++)
		{
			if (s[i + j] != '?' && t[j] != '?' && s[i + j] != t[j])
			{
				k++;
				break;
			}
		}
		if (k == 0)
			break;
	}
	for (j = i; j < ss; j++)
		if (a[j] == '?')
			a[j] = t[j - i];
	if (i + tt > ss)
	{
		for (j = ss; j < i + tt; j++)
			a[j] = t[j - i];
	}
	int aa = strlen(a);
	for (i = 0; i <= aa; i++)
	{
		k = 0;
		for (j = 0; j < min(rr, aa - i); j++)
		{
			if (a[i + j] != '?' && r[j] != '?' && a[i + j] != r[j])
			{
				k++;
				break;
			}
		}
		if (k == 0)
			break;
	}
	if (i + rr > aa)
	{
		for (j = aa; j < i + rr; j++)
			a[j] = r[j - i];
	}
	return strlen(a);
}
int main()
{
	char a[2003], b[2003], c[2003];
	scanf("%s%s%s", a, b, c);
	int ans = 10004;
	ans = min(ans, f(a, b, c));
	ans = min(ans, f(a, c, b));
	ans = min(ans, f(b, a, c));
	ans = min(ans, f(b, c, a));
	ans = min(ans, f(c, a, b));
	ans = min(ans, f(c, b, a));
	printf("%d\n", ans);
	return 0;
}