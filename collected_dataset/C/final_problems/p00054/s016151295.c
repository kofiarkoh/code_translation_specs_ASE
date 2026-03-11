#include <stdio.h>
int main() {
	int a, b, n, sum, mod, div;

	while (scanf(" %d %d %d ", &a, &b, &n) != EOF) {
		sum = 0;
		mod = a % b;
		div = a / b;
		while (n--) {
			a = mod * 10;
			mod = a % b;
			div = a / b;
			sum += div;
		}
		printf("%d\n", sum);
	}
	return 0;
}