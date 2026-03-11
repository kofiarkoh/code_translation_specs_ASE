// AOJ 2590: Unknown Switches
// 2017.11.30 bal4u@uu

#include <stdio.h>
#include <string.h>

char sw[1002][38];
char bul[1002][1002];
char can[38][1002];

int main()
{
	int n, m, q, i, j, k, c, cnt;
	char b1[40], b2[1010], *p;

	while (scanf("%d%d%d", &n, &m, &q) && n > 0) {
		memset(sw, 0, sizeof(sw));
		memset(bul, 0, sizeof(bul));
	    memset(can, 1, sizeof(can));
		for (i = 0; i < q; i++) {
			scanf("%s%s", b1, b2);
			for (p = b1, j = 0; j < n; j++) {
				sw[i][j] = *p++ & 1;
				if (i > 0) sw[i][j] ^= sw[i-1][j];
			}
			for (p = b2, j = 0; j < m; j++) bul[i][j] = *p++ & 1;
	    }

	    for (i = 0; i < q; i++) for (j = 0; j < n; j++) for (k = 0; k < m; k++)
			can[j][k] &= (sw[i][j] == bul[i][k]);

		for (i = 0; i < m; i++) {
			for (cnt = 0, j = 0; j < n; j++) if (can[j][i]) cnt++, k = j;
			if (cnt > 1) c = '?';
			else if (k < 10) c = k + '0';
			else             c = k -10 + 'A';
			putchar(c);
		}
		putchar('\n');
	}
	return 0;
}