#include <stdio.h>

int main(void)
{
	int a, b, c, d;
	
	while (scanf("%d", &a) != EOF) {
		b = 0;
		for (c = 1; c <= a; c++) {
			b += c;
		}
		printf("%d\n", b + 1);
	}
	
	return (0);
}
	