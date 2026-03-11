// AOJ 2320: Infinity Maze
// 2017.10.4 bal4u@uu

#include <stdio.h>
#include <string.h>

char map[102][102];
int mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};
int tr[128];
int mk[102][102][4];

int main()
{
	int h, w, r, c, d, r2, c2, sr, sc, step;
	long long l;
	char dir[5] = "NESW";

	tr['#'] = 10, tr['.'] = 11, tr['N'] = 0, tr['E'] = 1, tr['S'] = 2, tr['W'] = 3;
	while (scanf("%d%d%lld", &h, &w, &l) && l > 0) {
		memset(mk, 0, sizeof(mk));
		for (sr = -1, r = 0; r < h; r++) {
			scanf("%s", map[r]);
			if (sr >= 0) continue;
			for (c = 0; c < w; c++) {
				if (tr[map[r][c]] < 4) sr = r, sc = c, d = tr[map[r][c]], map[r][c] = '.';
			}
		}
		r = sr, c = sc, step = 0;
		while (l > 0) {
			if (mk[r][c][d]) {
				l = l % (step - mk[r][c][d]);
				if (!l) break;
			}
			mk[r][c][d] = step;
			while (1) {
				r2 = r + mv[d][0], c2 = c + mv[d][1];
				if (r2 >= 0 && r2 < h && c2 >= 0 && c2 < w && map[r2][c2] == '.') break;
				if (++d == 4) d = 0;
			}
			r = r2, c = c2;
			l--, step++;
		}
		printf("%d %d %c\n", r+1, c+1, dir[d]);
	}
    return 0;
}