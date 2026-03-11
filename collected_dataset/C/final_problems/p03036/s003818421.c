#include <stdio.h>

int main() {
	int r, d, x, y, n;

	scanf("%d%d%d", &r, &d, &x);
	n = 10;
	while (n--) {
		y = r * x - d;
		printf("%d\n", y);
		x = y;
	}
	return 0;
}
