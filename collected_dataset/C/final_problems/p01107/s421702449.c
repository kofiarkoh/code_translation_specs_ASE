// AOJ 1622 Go around the Labyrinth
// 2018.2.13 bal4u

#include <stdio.h>
#include <string.h>

typedef struct { char r, c, d, k; } STACK;
STACK sk[3000]; int top;
char map[53][53];
char mk[53][53];
int mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};


int check(int h, int w)
{
	int i, r, c, d, k, nr, nc;

	memset(mk, 0, sizeof(mk));
	sk[0].r = 1, sk[0].c = 1, sk[0].d = 1, sk[0].k = 0, top = 1;
	while (top) {
		r = sk[--top].r, c = sk[top].c, d = sk[top].d, k = sk[top].k;
		if (mk[r][c]) continue;
		mk[r][c] = 1;
		if      (k == 0 && r == 1 && c == w) k = 1;
		else if (k == 1 && r == h && c == w) k = 2;
		else if (k == 2 && r == h && c == 1) k = 3;
		else if (k == 3 && r == 2 && c == 1) return 1;

		d += 3; if (d >= 4) d -= 4;
		for (i = 0; i < 4; i++) {
			if (--d < 0) d = 3; 
			nr = r + mv[d][0], nc = c + mv[d][1];
			if (map[nr][nc] && !mk[nr][nc]) {
				sk[top].r = nr, sk[top].c = nc, sk[top].d = d, sk[top++].k = k;
			}
		}
	}
	return 0;
}

int main()
{
	int h, w, r, c;
	char buf[10], *p;

	while (fgets(buf, 10, stdin) && *buf != '0') {
		sscanf(buf, "%d%d", &h, &w);
		memset(map, 0, sizeof(map));
		for (r = 1; r <= h; r++) {
			fgets(p=map[r]+1, 52, stdin);
			c = w; while (c--) *p = (*p == '.'), p++; *p = 0;
		}
		puts(check(h, w)? "YES": "NO");
	}
	return 0;
}

