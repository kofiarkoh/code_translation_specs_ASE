#include <stdio.h>

int main() {
	int n, k, x, d;

	scanf("%d%d", &n, &k);
	d = 0;
	x = 1;
	while (x < n)
		d++, x *= k;
	printf("%d\n", d);
	return 0;
}
