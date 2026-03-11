// Aizu Vol-2 0210: The Squares
// 2017.8.20

#include <stdio.h>

#define MAX 30

#define E 0
#define N 1
#define W 2
#define S 3
#define EXIT 125
#define WALL 126

typedef struct { int r, c, d, nr, nc; } MAN;
MAN man[MAX*2]; int n, safe;

int mov[4][4][3] = {
	{ {  1, 0, S }, {  0, 1, E }, { -1, 0, N }, {  0,-1, W } },
	{ {  0, 1, E }, { -1, 0, N }, {  0,-1, W }, {  1, 0, S } },
	{ { -1, 0, N }, {  0,-1, W }, {  1, 0, S }, {  0, 1, E } },
	{ {  0,-1, W }, {  1, 0, S }, {  0, 1, E }, { -1, 0, N } } };

char map[MAX+3][MAX+3]; int R, C;

int main()
{
	int r, c, d, p, i, j, k;
	int tt, rr, cc;


	while (scanf("%d%d", &C, &R) && C) {
		for (r = 1; r <= R; r++) scanf("%s", map[r]+1);
		for (r = 0; r <= R+1; r++) map[r][0] = map[r][C+1] = WALL;
		for (c = 0; c <= C+1; c++) map[0][c] = map[R+1][c] = WALL;
		for (n = 0, r = 1; r <= R; r++) for (c = 1; c <= C; c++) {
			p = map[r][c];
			if (p == '.' || p == '#' || p == 'X') {
				if      (p == '.') p = 0;
				else if (p == '#') p = WALL;
				else if (p == 'X') p = EXIT;
				map[r][c] = p;
				continue;
			}
			man[++n].r = r, man[n].c = c;
			if (p == 'E') man[n].d = E;
			else if (p == 'N') man[n].d = N;
			else if (p == 'W') man[n].d = W;
			else man[n].d = S;
			map[r][c] = n;
		}

		for (safe = 0, tt = 1; tt <= 180; tt++) {
			for (i = 1; i <= n; i++) {
				r = man[i].r, c = man[i].c, d = man[i].d;
				if (r == 0) continue;
				for (j = 0; j < 4; j++) {
					rr = r + mov[d][j][0], cc = c + mov[d][j][1];
					if (map[rr][cc] == EXIT || map[rr][cc] == 0) {
						man[i].d = mov[d][j][2];
						break;
					}
				}
			}
			for (i = 1; i <= n; i++) {
				r = man[i].r, c = man[i].c, d = man[i].d;
				if (r == 0) continue;
				rr = r + mov[d][1][0], cc = c + mov[d][1][1];
				if (map[rr][cc] == EXIT) {
					man[i].r = 0, map[r][c] = 0;
					if (++safe == n) goto DONE;
					continue;
				}
				man[i].nr = man[i].nc = 0;
				if (map[rr][cc] != 0) continue;		
				if ((k = map[rr][cc+1]) > 0 && k < EXIT) {
					if (man[k].d == W) {
						if (k == i) man[i].nr = rr, man[i].nc = cc;
						continue;
					}
				}
				if ((k = map[rr-1][cc]) > 0 && k < EXIT) {
					if (man[k].d == S) {
						if (k == i) man[i].nr = rr, man[i].nc = cc;
						continue;
					}
				}
				if ((k = map[rr][cc-1]) > 0 && k < EXIT) {
					if (man[k].d == E) {
						if (k == i) man[i].nr = rr, man[i].nc = cc;
						continue;
					}
				}
				if ((k = map[rr+1][cc]) > 0 && k < EXIT) {
					if (man[k].d == N) {
						if (k == i) man[i].nr = rr, man[i].nc = cc;
						continue;
					}
				}
			}
			for (i = 1; i <= n; i++) {
				if (man[i].r > 0 && man[i].nr > 0) {
					r = man[i].r, c = man[i].c, rr = man[i].nr, cc = man[i].nc;
					map[r][c] = 0;
					map[rr][cc] = i;
					man[i].r = rr, man[i].c = cc;
				}
			}
		}
		puts("NA");
		continue;
DONE:	printf("%d\n", tt);
	}
	return 0;
}