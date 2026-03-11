#include <stdio.h>

int main(void) {
	int P;
	if (scanf("%d", &P) != 1) return 1;
	printf("%d\n", P - P % 500);
	return 0;
}

