#include <stdio.h>

int N;
int game[128][6];

int search(int isSente, int left, int right, int ste, int gte) {
	int kateru = 0;
	int daseta = 0;
	if (isSente) {
		if (gte == 0) return 0;
		if ((ste >> (left - 1)) & 1) {
			kateru = kateru || !search(0, left - 1, right, ste & ~(1 << (left - 1)), gte);
			daseta = 1;
		}
		if ((ste >> (right + 1)) & 1) {
			kateru = kateru || !search(0, left, right + 1, ste & ~(1 << (right + 1)), gte);
			daseta = 1;
		}
	} else {
		if (ste == 0) return 0;
		if ((gte >> (left - 1)) & 1) {
			kateru = kateru || !search(1, left - 1, right, ste, gte & ~(1 << (left - 1)));
			daseta = 1;
		}
		if ((gte >> (right + 1)) & 1) {
			kateru = kateru || !search(1, left, right + 1, ste, gte & ~(1 << (right + 1)));
			daseta = 1;
		}
	}
	if (!daseta) {
		kateru = !search(!isSente, left, right, ste, gte);
	}
	return kateru;
}

int main(void) {
	int i, j;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		for (j = 0; j < 6; j++) {
			if (scanf("%d", &game[i][j]) != 1) return 1;
		}
	}
	for (i = 0; i < N; i++) {
		int ste = 0;
		for (j = 0; j < 6; j++) {
			ste |= 1 << game[i][j];
		}
		puts(search(1, 7, 7, ste, 0x3F7E & ~ste) ? "yes" : "no");
	}
	return 0;
}

/*

1111
32109876543210
11111101111110
 3   F   7   E
*/

