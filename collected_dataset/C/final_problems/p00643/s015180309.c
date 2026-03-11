// AOJ 1057: Rolling Dice
// 2017.10.21 bal4u@uu

#include <stdio.h>
#include <string.h>

typedef struct { char r, c, d[3]; int s; } Q;
Q q[100000]; int top, end;

char map[11][11];
int mk[11][11][6][6];
int mv[4][2] =  {{-1,0},{0,1},{1,0},{0,-1}};
int rot[4][3] = {{2,6,3},{4,2,1},{5,1,3},{3,2,6}};

int main()
{
	int h, w, sr, sc, gr, gc, i, r, c, s, r2, c2, s2, ans;
	char dice[7], dice2[7];

	while (scanf("%d%d", &h, &w) && h > 0) {
		memset(mk, 0, sizeof(mk));
		for (r = 0; r < h; r++) for (c = 0; c < w; c++) scanf("%d", &i), map[r][c]= i;
		scanf("%d%d%d%d", &sr, &sc, &gr, &gc);
		q[0].r = sr, q[0].c = sc, q[0].s = 0, q[0].d[0] = 1, q[0].d[1] = 2, q[0].d[2] = 3;
		ans = 0x7fffffff; top = 0, end = 1;
		while (top < end) {
			r = q[top].r, c = q[top].c, s = q[top].s;
			memcpy(dice+1, q[top].d, 3); for (i = 4; i <= 6; i++) dice[i] = 7 - dice[7-i]; top++;
			if (r == gr && c == gc) {
				if (s < ans) ans = s; continue;
			}
			for (i = 0; i < 4; i++) {
				r2 = r + mv[i][0], c2 = c + mv[i][1];
				if (r2 < 0 || r2 >= h || c2 < 0 || c2 >= w) continue;
				dice2[1] = dice[rot[i][0]],	dice2[2] = dice[rot[i][1]], dice2[3] = dice[rot[i][2]];
				s2 = s + (7 - dice2[1])*map[r2][c2];
				if (mk[r2][c2][dice2[1]][dice2[2]] != 0 && mk[r2][c2][dice2[1]][dice2[2]] < s2) continue;
				mk[r2][c2][dice2[1]][dice2[2]] = s2;
				q[end].r = r2, q[end].c = c2, q[end].s = s2;
				memcpy(q[end].d, dice2+1, 3); end++;
			}
		}
		printf("%d\n", ans);
	}
	return 0;
}