// Aizu Vol-2 0226: Hit and Blow
// 2017.8.6

#include <stdio.h>

int f[10];

int main()
{
	char r[5], a[5];
	int h, b, i;

	while (1) {
		scanf("%s%s", r, a);
		if (r[0] == '0' && !r[1]) break;
		for (h = i = 0; i < 4; i++) {
			if (r[i] == a[i]) h++, r[i] = a[i] = 0;
		}
		for (i = 0; i < 10; i++) f[i] = 0;
		for (i = 0; i < 4; i++) if (r[i]) f[r[i] & 0xf]++;
		for (b = i = 0; i < 4; i++) if (a[i] && f[a[i] & 0xf]) b++;
		printf("%d %d\n", h, b);
	}
	return 0;
}