#include <math.h>
#include <stdio.h>

int main() {
	int a, b, c, k, max;

	scanf("%d%d%d%d", &a, &b, &c, &k);
	max = 0;
	if (max < a)
		max = a;
	if (max < b)
		max = b;
	if (max < c)
		max = c;
	printf("%d\n", a + b + c * (int) pow(2, k));
	return 0;
}
