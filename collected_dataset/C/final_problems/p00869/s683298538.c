// AOJ 1290: Traveling Cube
// 2017.10.20 bal4u@uu

#include <stdio.h>
#include <string.h>

typedef struct { char r, c, d[3], n; int s; } Q;
Q q[100000]; int top, end;

#define RED     1		// top  r
#define MAGENTA 2		// south  m
#define BLUE    3		// east  b
#define YELLOW  4		// west  y
#define GREEN   5		// north  g
#define CYAN    6		// bottom c
#define WHITE   7		// w
#define BLACK   8       // k

char map[32][32];
char mk[32][32][6][6][6];
char seq[7];
int mv[4][2] =  {{-1,0},{0,1},{1,0},{0,-1}};
int rot[4][3] = {{2,6,3},{4,2,1},{5,1,3},{3,2,6}};
int tr['z'];

int main()
{
	int w, d, i, r, c, n, r2, c2, n2, s, ans;
	char dice[7], dice2[7];

	tr['r'] = RED, tr['m'] = MAGENTA, tr['b'] = BLUE, tr['c'] = CYAN, tr['g'] = GREEN, tr['y'] = YELLOW;
	tr['w'] = WHITE, tr['k'] = BLACK;
	while (scanf("%d%d", &w, &d) && w > 0) {
		memset(mk, 0, sizeof(mk));
		for (r = 0; r < d; r++) {
			scanf("%s", map[r]);
			for (c = 0; c < w; c++) {
				if (map[r][c] == '#') {
					q[0].r = r, q[0].c = c; map[r][c] = WHITE;
					q[0].d[0] = RED, q[0].d[1] = MAGENTA, q[0].d[2] = BLUE;
					q[0].n = 0, q[0].s = 0;
				} else map[r][c] = tr[map[r][c]];
			}
		}
		scanf("%s", seq); for (i = 0; i < 6; i++) seq[i] = tr[seq[i]];
		ans = -1; top = 0, end = 1;
		while (top < end) {
			r = q[top].r, c = q[top].c, n = q[top].n, s = q[top].s+1;
			memcpy(dice+1, q[top].d, 3); for (i = 4; i <= 6; i++) dice[i] = 7 - dice[7-i]; top++;
			for (i = 0; i < 4; i++) {
				r2 = r + mv[i][0], c2 = c + mv[i][1];
				if (r2 < 0 || r2 >= d || c2 < 0 || c2 >= w || map[r2][c2] == BLACK) continue;
				dice2[1] = dice[rot[i][0]],	dice2[2] = dice[rot[i][1]], dice2[3] = dice[rot[i][2]];
				if (map[r2][c2] == WHITE) n2 = n;
				else {
					if (dice2[1] != seq[n] || dice2[1] != map[r2][c2]) continue;
					if (n == 5) { ans = s; goto done; }
					n2 = n + 1;
				}
				if (mk[r2][c2][dice2[1]][dice2[2]][n]) continue;
				mk[r2][c2][dice2[1]][dice2[2]][n] = 1;
				q[end].r = r2, q[end].c = c2, q[end].n = n2, q[end].s = s;
				memcpy(q[end].d, dice2+1, 3); end++;
			}
		}
		if (ans < 0) puts("unreachable");
		else {
done:		printf("%d\n", ans);
		}
	}
	return 0;
}