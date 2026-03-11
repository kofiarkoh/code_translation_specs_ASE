// AOJ 2017 Karakuri Doll
// 2018.3.16 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define QMAX 300000
typedef struct { char r1, c1, d1, r2, c2, d2; } Q;
Q q[QMAX + 5]; int top, end;
int H, W, SR, SC, SD, GR, GC, GD;     // (r,c) dir
char map[16][66];
char visited[17][65][5][17][65][5];   // r1,c1,d1 r2,c2,d2
int mv[4][2] = { {-1,0},{0,1},{1,0},{0,-1} };
#define L(d) ((d+3)%4)
#define R(d) ((d+1)%4)
int ans;

void enq(int r1, int c1, int d1, int r2, int c2, int d2, int _d2)
{
	if (r1 == GR && c1 == GC) {
		ans = 1;
		if (r2 == GR && c2 == GC && _d2 == GD) { ans = 2; return; }
	}
	if (visited[r1][c1][d1][r2][c2][d2]) return;
	visited[r1][c1][d1][r2][c2][d2] = 1;

	q[end].r1 = r1, q[end].c1 = c1, q[end].d1 = d1;
	q[end].r2 = r2, q[end].c2 = c2, q[end].d2 = d2;
	++end;
}

int bfs()
{
	int r1, c1, d1, r2, c2, d2;

	memset(visited, 0, sizeof(visited));
	ans = 0, top = end = 0;
	for (d2 = 0; d2 < 4; d2++) enq(SR, SC, SD, SR, SC, d2, 4);

	while (top != end && ans != 2) {
		r1 = q[top].r1, c1 = q[top].c1, d1 = q[top].d1;
		r2 = q[top].r2, c2 = q[top].c2, d2 = q[top].d2;
		++top;

		while (map[r1 + mv[d1][0]][c1 + mv[d1][1]] != '#')
			r1 += mv[d1][0], c1 += mv[d1][1];
		enq(r1, c1, L(d1), H, W, 4, 4);
		enq(r1, c1, R(d1), H, W, 4, 4);

		if (d2 < 4) while (map[r2][c2] != '#') {
			if (map[r2 + mv[L(d2)][0]][c2 + mv[L(d2)][1]] == '#')
				enq(r1, c1, L(d1), r2, c2, L(d2), d2);
			if (map[r2 + mv[R(d2)][0]][c2 + mv[R(d2)][1]] == '#')
				enq(r1, c1, R(d1), r2, c2, R(d2), d2);
			r2 -= mv[d2][0], c2 -= mv[d2][1];
		}
	}
	return ans;
}

int main()
{
	int r, c;
	static char *msg[3] = { "He cannot bring tea to his master.",
							"He cannot return to the kitchen.",
							"He can accomplish his mission." };

	while (scanf("%d%d", &W, &H) && (W | H)) {
		for (r = 0; r < H; r++) {
			scanf("%s", map[r]);
			for (c = 0; c < W; c++) {
				if (map[r][c] == 'K') SR = r, SC = c;
				else if (map[r][c] == 'M') GR = r, GC = c;
			}
		}
		for (SD = 0; ; SD++) {
			r = SR + mv[SD][0], c = SC + mv[SD][1];
			if (map[r][c] != '#') break;
		}
		for (GD = 0; ; GD++) {
			r = GR + mv[GD][0], c = GC + mv[GD][1];
			if (map[r][c] != '#') break;
		}
		printf("%s\n", msg[bfs()]);
	}
	return 0;
}
