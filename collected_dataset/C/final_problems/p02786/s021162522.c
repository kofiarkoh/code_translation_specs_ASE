#include<stdio.h>
int main()
{
	long long int h;
	scanf("%lld", &h);
	int c = 0;
	while (h > 0)
	{
		c++;
		h /= 2;
	}
	long long int ans = 1, v = 2;
	while (c > 0)
	{
		if (c % 2 > 0)
			ans = ans * v;
		v = v * v;
		c /= 2;
	}
	printf("%lld\n", ans - 1);
	return 0;
}