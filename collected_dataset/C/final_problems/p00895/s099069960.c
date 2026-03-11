// AOJ 1316: The Sorcerer's Donut
// 2017.12.4 bal4u@uu

#include <stdio.h>
#include <string.h>

#define HASHSIZ 500009
typedef struct { unsigned x; int w; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash+HASHSIZ;			// Hash table
char ans[500]; int len;
char buf[1000];

void lookup(unsigned x, int w)
{
	int i = (int)(x % HASHSIZ);
	HASH *tp = hash + i;

	while (tp->x) {
		if (w == tp->w && x == tp->x) {
			if (w > len || (w == len && memcmp(buf, ans, w) < 0))
				memcpy(ans, buf, w), len = w;
			return;
		}
		if (++tp == hashend) tp = hash;
	}
	tp->x = x, tp->w = w;
}

#define M 4294967291
int h, w, sr, sc, dir;
char m[10][22];
int mv[8][2] = {{1,0},{1,1},{0,1},{-1,1},{-1,0},{-1,-1},{0,-1},{1,-1}};


void rec(int r, int c, unsigned x, int len)
{
	int r2, c2;

	buf[len++] = m[r][c];
	x = (((long long)x << 5) | (m[r][c]-0x40)) % M;
	lookup(x, len);

	r2 = r + mv[dir][0]; if (r2 < 0) r2 = h-1; else if (r2 >= h) r2 = 0;
	c2 = c + mv[dir][1]; if (c2 < 0) c2 = w-1; else if (c2 >= w) c2 = 0;

	if (r2 == sr && c2 == sc) return;
	rec(r2, c2, x, len);
}

int main()
{
	int r;

	while (scanf("%d%d", &h, &w) && h > 0) {
		memset(hash, 0, sizeof(hash));
		for (r = 0; r < h; r++) scanf("%s", m[r]);

		len = 0;
		for (sr = 0; sr < h; sr++) for (sc = 0; sc < w; sc++) for (dir = 0; dir < 8; dir++) {
			rec(sr, sc, 0, 0);
		}

		if (len < 2) puts("0");
		else ans[len] = 0, puts(ans);
	}
	return 0;
}