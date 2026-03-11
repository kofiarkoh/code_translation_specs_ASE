// AOJ 2447 A Two Floors Dungeon
// 2018.2.2 bal4u

#include <stdio.h>
#include <ctype.h>
#include <string.h>

#define QMAX 800000
typedef struct { short s; char f, r, c; } Q;
Q q[QMAX+10]; int top, end;

int S;
char map[53][53];
char sw[11][53][53];
int mov[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};
int memo[1024][2][53][53];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int check(int s, int f, int r, int c)
{
	int i, g;

	if (map[r][c] == '|') return 1;
	g = map[r][c] == '^' || isupper(map[r][c]);
	for (i = 0; i < S; i++) if ((s >> i) & 1) g ^= sw[i][r][c];
	return g == f;
}

void enq(int ns, int nf, int nr, int nc, int k)
{
	if (memo[ns][nf][nr][nc]) return;
	memo[ns][nf][nr][nc] = k;
	q[end].s = ns, q[end].f = nf, q[end].r = nr, q[end++].c = nc;
}

int calc(int sr, int sc)
{
	int i, k, s, f, r, c, nr, nc;

	q[0].r = sr, q[0].c = sc, top = 0, end = 1;
	memo[0][0][sr][sc] = 1;
	while (top != end) {
		if (end >= QMAX) break;
		s = q[top].s, f = q[top].f, r = q[top].r, c = q[top++].c;
		k = memo[s][f][r][c] + 1;
		if (map[r][c] == '&') return k-2;
		if (map[r][c] == '|') enq(s, f ^ 1, r, c, k);
		if (isalpha(map[r][c])) {
			i = (map[r][c] & 0x1f) - 1;
			enq(s ^ (1 << i), f ^ sw[i][r][c], r, c, k);
		}
		for (i = 0; i < 4; i++) {
			nr = r + mov[i][0], nc = c + mov[i][1];
			if (map[nr][nc] == '#') continue;
			if (check(s, f, nr, nc)) enq(s, f, nr, nc, k);
		}
	}
	return -1;
}

int main()
{
	int W, H, i, r, c, sr, sc; 
	char buf[53];

	W = in(), H = in();
	sr = -1;
	for (r = 0; r < H; r++) {
		fgets(map[r], 53, stdin);
		if (sr >= 0) continue;
		for (c = 0; c < W; c++) {
			if (map[r][c] == '%') sr = r, sc = c;
		}
	}

	S = in();
	for (i = 0; i < S; i++) {
		for (r = 0; r < H; r++) {
			fgets(buf, 53, stdin);
			for (c = 0; c < W; c++) {
				if (buf[c] == '*') sw[i][r][c] = 1;
			}
		}
	}
	printf("%d\n", calc(sr, sc));
	return 0;
}
