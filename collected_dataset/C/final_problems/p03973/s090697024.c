#include<stdio.h>
int main()
{
	int n;
	scanf("%d", &n);
	int i;
	long long int a[100005];
	for (i = 0; i < n; i++)
		scanf("%lld", &a[i]);
	long long int ans = a[0] - 1;
	long long int v = 2;
	for (i = 1; i < n; i++)
	{
		ans += a[i] / v;
		if (a[i] % v == 0)
			ans--;
		if (a[i] == v)
			v++;
	}
	printf("%lld\n", ans);
}