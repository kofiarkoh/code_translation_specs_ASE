#include <stdio.h>

int n, m;
int p[64][1024];

int main(void) {
	while (scanf("%d%d", &n, &m) == 2 && (n|m) != 0) {
		int i, j;
		int gotiusa = 0, kinmoza;
		for (i = 0; i < m; i++) {
			for (j = 0; j < n; j++) {
				if (scanf("%d", &p[i][j]) != 1) return 1;
			}
		}
		for (i = 0; i < n; i++) {
			kinmoza = 0;
			for (j = 0; j < m; j++) {
				kinmoza += p[j][i];
			}
			if (kinmoza > gotiusa) gotiusa = kinmoza;
		}
		printf("%d\n", gotiusa);
	}
	return 0;
}

