#include <stdio.h>

int main(void)
{
	int n;
	long long int a[30000];
	long long int all = 0;
	int i;

	scanf("%d", &n);
	for (i = 0; i < n; i++)
	{
		scanf("%lld", &a[i]);
		all ^= a[i];
	}

	for (i = 0; i < n; i++)
	{
		printf("%lld ", all ^ a[i]);
	}
	putchar('\n');

	return 0;
}
