// AOJ 1622 Go around the Labyrinth
// 2018.2.13 bal4u

#include <stdio.h>
#include <string.h>

typedef struct { char r, c; } Q;
Q q[2000]; int top;
int h, w;
char map[53][53];
int mv4[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};
int mv9[9][2] = {{-1,-1},{-1,0},{-1,1},{0,-1},{0,0},{0,1},{-1,1},{-1,0},{1,1}};
char mk[53][53];

int check()
{
	int i, r, c, nr, nc, f;

	memset(mk, 0, sizeof(mk));
	q[0].r = 1, q[0].c = 1, top = 1;
	mk[1][1] = 1;
	f = 0;
	while (top) {
		r = q[--top].r, c = q[top].c;
		if      (r == 1) { if (c == w) f++; }
		else if (r == h) {
			if      (c == 1) f++;
		    else if (c == w) f++;
		}
		if (f == 3) return 0;
		for (i = 0; i < 4; i++) {
			nr = r + mv4[i][0], nc = c + mv4[i][1];
			if (!map[nr][nc] || mk[nr][nc]) continue;
			mk[nr][nc] = 1;
			q[top].r = nr, q[top++].c = nc;
		}
	}
	return 1;
}

int main()
{
	int r, c, i, f, nr, nc;
	char buf[10], *p;

	while (fgets(buf, 10, stdin) && *buf != '0') {
		sscanf(buf, "%d%d", &h, &w);
		memset(map, 0, sizeof(map));
		for (r = 1; r <= h; r++) {
			fgets(p = map[r]+1, 52, stdin);
			c = w; while (c--) *p = (*p == '.'), p++; *p = 0;
		}

		if (check()) goto no;
		for (r = 1; r <= h; r++) for (c = 1; c <= w; c++) {
			if (r == 1 && c == 1 || r == 1 && c == w ||
				r == h && c == 1 || r == h && c == w) continue;
			if (!map[r][c]) continue;
			f = 0;
			for (i = 0; i < 9; i++) {
				nr = r + mv9[i][0], nc = c + mv9[i][1];
				if (!map[nr][nc]) { f = 1; break; }
			}
			if (f) {
				map[r][c] = 0;
				if (check()) goto no;
				map[r][c] = 1;
			}
		}
		puts("YES"); continue;
no:		puts("NO");
	}
	return 0;
}

