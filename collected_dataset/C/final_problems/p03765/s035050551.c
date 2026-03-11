#include<stdio.h>
int main()
{
	char s[100005], t[100005];
	scanf("%s%s", s, t);
	int q;
	scanf("%d", &q);
	int a[100005], b[100005], c[100005], d[100005];
	int i;
	for (i = 0; i < q; i++)
		scanf("%d %d %d %d", &a[i], &b[i], &c[i], &d[i]);
	int vs[100005], vt[100005];
	vs[0] = 0;
	for (i = 0; s[i] != '\0'; i++)
	{
		if (s[i] == 'A')
			vs[i + 1] = vs[i] + 1;
		else
			vs[i + 1] = vs[i] + 2;
	}
	vt[0] = 0;
	for (i = 0; t[i] != '\0'; i++)
	{
		if (t[i] == 'A')
			vt[i + 1] = vt[i] + 1;
		else
			vt[i + 1] = vt[i] + 2;
	}
	for (i = 0; i < q; i++)
	{
		if ((vs[b[i]] - vs[a[i] - 1]) % 3 != (vt[d[i]] - vt[c[i] - 1]) % 3)
			printf("NO\n");
		else
			printf("YES\n");
	}
	return 0;
}