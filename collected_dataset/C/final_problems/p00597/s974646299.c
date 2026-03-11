// Aizu 1011: Finding the Largest Carbon Compound Give...
// 2017.9.17 bal4u@uu

#include <stdio.h>

int a[31];

int main()
{
	int i;
	a[1] = 1, a[2] = 2;
	for (i = 3; i <= 30; i++) a[i] = 3*a[i-2] + 2;
	while (scanf("%d", &i) == 1) printf("%d\n", a[i]);
	return 0;
}