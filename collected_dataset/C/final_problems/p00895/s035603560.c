// AOJ 1316: The Sorcerer's Donut
// 2017.12.4 bal4u@uu

#include <stdio.h>
#include <string.h>

#define HASHSIZ 500009
typedef struct { unsigned x; int w; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash+HASHSIZ;			// Hash table
char ans[500]; int len;

void lookup(unsigned x, char *s, int w)
{
	int i = (int)(x % HASHSIZ);
	HASH *tp = hash + i;

	while (tp->x) {
		if (w == tp->w && x == tp->x) {
			if (w > len || (w == len && memcmp(s, ans, w) < 0))
				memcpy(ans, s, w), len = w;
			return;
		}
		if (++tp == hashend) tp = hash;
	}
	tp->x = x, tp->w = w;
}

#define M 4294967291
int h, w;
char m[10][22];
int mv[8][2] = {{1,0},{1,1},{0,1},{-1,1},{-1,0},{-1,-1},{0,-1},{1,-1}};
char buf[1000];

void rec(int sr, int sc, int r, int c, int dir, char *s,
		 unsigned x, int len)
{
	int r2, c2, cc;

	s[len++] = m[r][c];
	cc = m[r][c] - ('A'-1);
	x = (((long long)x << 5) | cc) % M;
	lookup(x, s, len);

	r2 = r + mv[dir][0]; if (r2 < 0) r2 = h-1; else if (r2 >= h) r2 = 0;
	c2 = c + mv[dir][1]; if (c2 < 0) c2 = w-1; else if (c2 >= w) c2 = 0;

	if (r2 == sr && c2 == sc) return;
	rec(sr, sc, r2, c2, dir, s, x, len);
}

int main()
{
	int r, c, d;

	while (scanf("%d%d", &h, &w) && h > 0) {
		memset(hash, 0, sizeof(hash));
		for (r = 0; r < h; r++) scanf("%s", m[r]);

		len = 0, ans[0] = 0;
		for (r = 0; r < h; r++) for (c = 0; c < w; c++) for (d = 0; d < 8; d++) {
			rec(r, c, r, c, d, buf, 0, 0);
		}

		if (len < 2) puts("0");
		else {
			ans[len] = 0;
			puts(ans);
		}
	}
	return 0;
}