// AOJ 1214: Walking Ant
// 2017.10.15

#include <stdio.h>
#include <string.h>

typedef struct { int r, c, t, hp; } Q;
Q q[100]; int top;
char map[10][10];
char mk[10][10][7];

int mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};

int main()
{
	int w, h, i, a, r, c, t, hp, r2, c2, sr, sc, gr, gc, ans;

	while (scanf("%d%d", &w, &h) && w > 0) {
		memset(mk, 0, sizeof(mk));
		for (r = 0; r < h; r++) for (c = 0; c < w; c++) {
			scanf("%d", &a);
			map[r][c] = a;
			if (a == 2) sr = r, sc = c, map[r][c] = 0;
			else if (a == 3) gr = r, gc = c;
		}
		q[0].r = sr, q[0].c = sc, q[0].t = 0, q[0].hp = 6, top = 1;
		mk[sr][sc][6] = 1;
		ans = -1;
		while (top) {
			r = q[--top].r, c = q[top].c, t = q[top].t, hp = q[top].hp;
			if (r == gr && c == gc) { ans = t; break; }
			for (i = 0; i < 4; i++) {
				r2 = r + mv[i][0], c2 = c + mv[i][1];
				if (r2 < 0 || r2 >= h || c2 < 0 || c2 >= w || map[r2][c2] == 0) continue;
				if (mk[r2][c2][hp]) continue;
				if (hp <= 1) continue;
				mk[r2][c2][hp] = 1;
				q[top].r = r2, q[top].c = c2, q[top].t = t+1;
				q[top].hp = (map[r2][c2] == 4) ? 6 : hp-1;
				top++;
			}
		}
		printf("%d\n", ans);
	}
	return 0;
}