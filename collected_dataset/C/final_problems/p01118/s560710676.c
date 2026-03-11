#include <stdio.h>
#include <stdlib.h>

int h, w;
char r[64][64];
char s[1024];

int pos[256][2];

int main(void) {
	while (scanf("%d%d", &h, &w) == 2 && (h|w) != 0) {
		int i, j;
		int mizuhasu = 0;
		for (i = 0; i < h; i++) {
			if (scanf("%63s", r[i]) != 1) return 1;
		}
		if (scanf("%1022s", s + 1) != 1) return 1;
		for (i = 0; i < h; i++) {
			for (j = 0; j < w; j++) {
				int idx = (unsigned char)r[i][j];
				pos[idx][0] = i;
				pos[idx][1] = j;
			}
		}
		for (i = 1; s[i] != '\0'; i++) {
			int current = (unsigned char)s[i], prev = (unsigned char)s[i - 1];
			mizuhasu += abs(pos[current][0] - pos[prev][0]) + abs(pos[current][1] - pos[prev][1]) + 1;
		}
		printf("%d\n", mizuhasu);
	}
	return 0;
}

