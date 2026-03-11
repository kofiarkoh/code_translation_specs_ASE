#include <stdio.h>
#include <math.h>

int main(void) {
	int a, b, c, x, y, money=0;
	scanf("%d %d %d %d %d", &a, &b, &c, &x, &y);
	if (2 * c < a + b) {
		if (x < y) {
			money += c*x * 2;
			money += b*(y - x);
			if (money > c*y * 2)money = c*y * 2;
		}
		else {
			money += c*y * 2;
			money += a*(x - y);
			if (money > c*x * 2)money = c*x * 2;
		}
	}
	else {
		money += a*x + b*y;
	}
	printf("%d\n", money);
	return 0;
}