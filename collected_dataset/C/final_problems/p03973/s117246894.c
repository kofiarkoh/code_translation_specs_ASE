#include<stdio.h>
int main()
{
	int n;
	scanf("%d", &n);
	int i;
	int a[100005];
	for (i = 0; i < n; i++)
		scanf("%d", &a[i]);
	int ans = 0;
	int max = 0;
	for (i = 0; i < n; i++)
	{
		if (a[i] == max + 1)
			max++;
		else
			ans += (a[i] - 1) / (max + 1);
		if (i == 0)
			max = 1;
	}
	printf("%d\n", ans);
}