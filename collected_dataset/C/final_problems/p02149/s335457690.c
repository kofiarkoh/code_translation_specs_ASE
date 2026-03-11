#include <stdio.h>

int main(void) {
	int a, b, c;
	if (scanf("%d%d%d", &a, &b, &c) != 3) return 1;
	if (a > b && a > c) puts("A");
	else if (a < b && b > c) puts("B");
	else puts("C");
	return 0;
}

