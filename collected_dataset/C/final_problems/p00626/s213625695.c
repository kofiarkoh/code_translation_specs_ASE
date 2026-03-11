// AOJ 1040 Chocolate with Heart Marks
// 2018.2.6 bal4u
 
#include <stdio.h>
#include <string.h>

#define N 144		// 12x12
#define INF 0x03030303
int s[10];
int g[N][N];
int mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	while (c < '0') c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int b[128][N];
int minimumSteinerTree(int *s, int sz, int g[N][N], int gsz)
{
	int i, j, k, t, lim, ans;

    if (sz <= 1) return 0;

	for (k = 0; k < gsz; k++) for (i = 0; i < gsz; i++) for (j = 0; j < gsz; j++) {
		t = g[i][k] + g[k][j];
		if (t < g[i][j]) g[i][j] = t;
	}

	memset(b, INF, sizeof(b));
	for (t = 1, i = 0; i < sz; i++, t <<= 1) {
		for (j = 0; j < gsz; j++) b[t][j] = g[s[i]][j];
	}

	lim = (1<<sz) - 1;
	for (i = 1; i <= lim; i++) {
		if (!(i & (i-1))) continue;
		for (j = 0; j < gsz; j++) for (k = 0; k < i; k++) {
			if ((i | k) == i) {
				t = b[k][j] + b[i-k][j];
				if (t < b[i][j]) b[i][j] = t;
			}
		}
		for (j = 0; j < gsz; j++) for (k = 0; k < gsz; k++) {
			t = b[i][k] + g[j][k];
			if (t < b[i][j]) b[i][j] = t;
		}
	}

	ans = INF;
    for (i = 0; i <= lim; i++) for (j = 0; j < gsz; j++) {
		t = b[i][j] + b[lim-i][j];
		if (ans > t) ans = t;
	}
    return ans;
}

int main()
{
    int H, W, i, r, c, t, nr, nc, nt, sz;

    while (H = in()) {
		W = in();
		memset(g, INF, sizeof(g));
		sz = 0;
		for (r = 0; r < H; r++) for (c = 0; c < W; c++) {
			t = r*W + c;
			if (in()) s[sz++] = t;

			for (i = 0; i < 4; i++) {
				nr = r + mv[i][0], nc = c + mv[i][1];
				if (nr < 0 || nr >= H || nc < 0 || nc >= W) continue;
				nt = nr*W + nc;
				g[t][nt] = 1;
			}
			g[t][t] = 0;
		}

		printf("%d\n", H*W - minimumSteinerTree(s, sz, g, H*W) - 1);
	}
	return 0;
}

