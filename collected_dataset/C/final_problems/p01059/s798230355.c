// AOJ 1574: Gossip
// 2017.10.23 bal4u@uu

#include <stdio.h>

int main() 
{
	int n, m, d, a, a0, a1;

	scanf("%d%d", &n, &m);
	scanf("%d", &a1), a0 = a1, d = 0;
	while (--m) {
		scanf("%d", &a);
		if (a - a0 > d) d = a - a0;
		a0 = a;
	}
	d /= 2;
	if (a1 - 1 > d) d = a1 - 1;
	if (n - a0 > d) d = n - a0;
	printf("%d\n", d);
	return 0; 
}