// AOJ 2610: Fast Division
// 2017.10.6 bal4u@uu

#include <stdio.h>

int ans[4] = { 1, 2, 1, 0 };
int main()
{
	int n;

	scanf("%d", &n);
	if (n > 2) n = 3;
	printf("%d\n", ans[n]);
	return 0;
}