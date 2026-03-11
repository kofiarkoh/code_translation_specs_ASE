// Aizu 3013: Clock
// 2017.9.22 bal4u@uu

#include <stdio.h>

int main(void)
{
	int a, h, m;

	scanf("%d", &a);
	a <<= 1;
	m = a % 60;
	h = (a - m) / 60;
	printf("%d %d\n", h, m);
	return 0;
}