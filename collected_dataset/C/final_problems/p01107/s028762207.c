// AOJ 1622 Go around the Labyrinth
// 2018.2.13 bal4u

#include <stdio.h>
#include <string.h>

typedef struct { char r, c, d, k; } S;
S s[3000]; int top;
char map[53][53];
char mk[53][53];
int mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};

int check(int h, int w)
{
	int i, r, c, d, k, nr, nc;

	memset(mk, 0, sizeof(mk));
	s[0].r = 1, s[0].c = 1, s[0].d = 1, s[0].k = 1, top = 1;
	while (top) {
		r = s[--top].r, c = s[top].c, d = s[top].d, k = s[top].k;

		if (mk[r][c]) continue;
		mk[r][c] = 1;

		if      ((k & 1) && r == h && c == 1) k |= 2;
		else if ((k & 2) && r == h && c == w) k |= 4;
		else if ((k & 4) && r == 1 && c == w) k |= 8;
		else if ((k & 8) && r == 1 && c == 2) return 1;

		d += 2; if (d >= 4) d -= 4;
		for (i = 0; i < 4; i++) {
			if (++d == 4) d = 0; 
			nr = r + mv[d][0], nc = c + mv[d][1];
			if (map[nr][nc] && !mk[nr][nc]) {
				s[top].r = nr, s[top].c = nc, s[top].d = d, s[top++].k = k;
			}
		}
	}
	return 0;
}

int main()
{
	int h, w, r, c;
	char b[10], *p;

	while (fgets(b, 10, stdin) && *b != '0') {
		sscanf(b, "%d%d", &h, &w);
		for (r = 1; r <= h; r++) {
			fgets(p=map[r]+1, 52, stdin);
			c = w; while (c--) *p = (*p == '.'), p++; *p = 0;
		}
		memset(map[r], 0, w+2); 
		puts(check(h, w)? "YES": "NO");
	}
	return 0;
}

