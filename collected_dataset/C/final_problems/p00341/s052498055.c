// Aizu Vol-3 0346: Rectangle Made with Bars
// 2017.8.25

#include <stdio.h>

int e[12], f[12];

int main()
{
	int i, j, k;

	for (i = 0; i < 12; i++) e[i] = -10000, f[i] = 0;
	for (i = 0; i < 12; i++) { 
		scanf("%d", &k);
		for (j = 0; j < 12; j++) {
			if (k == e[j]) { f[j]++; break; } 
			else if (e[j] < 0) { e[j] = k, f[j] = 1; break; }
		}
	}
	for (i = 0; i < 6; i++) if (f[i] & 1) break;
	puts(i >= 6 ? "yes" : "no");
}