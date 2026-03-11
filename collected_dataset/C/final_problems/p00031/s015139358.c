// Aizu Vol0 0031: Weight
// 2017.7.29

#include <stdio.h>
#include <stdlib.h>

char *gets(char *);

int w[10] = { 1, 2, 4, 8, 16, 32, 64, 128, 256, 512 };

int main()
{
	int i, x, f;
	char buf[100];

	while (gets(buf) != NULL) {
		x = atoi(buf);
		for (i = 0, f = 1; x > 0; i++, x >>= 1) {
			if (x & 1) {
				if (f) f = 0;
				else putchar(' ');
				printf("%d", w[i]);
			}
		}
		putchar('\n');
	}
	return 0;
}