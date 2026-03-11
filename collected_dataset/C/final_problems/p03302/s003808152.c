#include <stdio.h>

int main() {
	int a, b;
	scanf("%d", &a);
	scanf("%d", &b);
	if (a + b == 15)printf("+");
	else if (a*b == 15)printf("*");
	else printf("x");

	return 0;
}