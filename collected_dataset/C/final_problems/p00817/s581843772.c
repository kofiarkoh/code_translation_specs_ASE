// AOJ 1238 True Liars
// 2018.3.20 bal4u

#include <stdio.h>
#include <string.h>

#define MAX 603

// [library]

#define gch() getchar_unlocked()
//#define gch() getchar()

int in()			// get a non-negative integer from stdin
{
	int n = 0, c = gch();
	do n = 10*n + (c & 0xf), c = gch(); while (c >= '0');
	return n;
}

/* UNION-FIND */
int id[2*MAX], size[2*MAX];
void init(int n) { int i; for (i = 0; i < n; i++) id[i] = i, size[i] = 1; }
int root(int i) { while (i != id[i]) id[i] = id[id[i]], i = id[i]; return i; }
int connected(int p, int q) { return root(p) == root(q); }
void unite(int p, int q)
{
    int i = root(p), j = root(q); if (i == j) return;
    if (size[i] < size[j]) id[i] = j, size[j] += size[i]; else id[j] = i, size[i] += size[j];
}

// [\library]

int   sz;
short f[MAX][MAX], sf[MAX];
short g[MAX][MAX], sg[MAX];
short belong[MAX];

short dp[MAX][MAX];
short which[MAX][MAX];
char  divine[MAX];

int main()
{
	int n, p1, p2, p, i, j, k, x, xx, y, yy, r;

	while (1) {
		n = in(), p1 = in(), p2 = in(), p = p1+p2;
		if ((n|p1|p2) == 0) break;
		if (n == 0) {
			if (p1 == 0) puts("end");
			else if (p2 == 0) { for (i = 1; i <= p; i++) printf("%d\n", i); puts("end"); } 
			else puts("no");
			continue;
		}

		init(p << 1);

		while (n--) {
			x = in()-1, xx = x<<1, y = in()-1, yy = y<<1, r = gch();
			if (r == 'y') {
				unite(xx, yy), unite(xx+1, yy+1); gch(), gch(), gch();
			} else {
				unite(xx, yy+1), unite(xx+1, yy); gch(), gch();
			}
		}

		sz = 1, memset(belong, 0, p<<1);
		memset(sf, 0, p<<1), memset(sg, 0, p<<1);
		for (x = 0; x < p; x++) {
			xx = x<<1, i = root(xx);
			if (i == xx) i = root(++xx);

			j = belong[i>>1];
			if (j == 0) j = belong[i>>1] = sz++;
			if ((xx & 1) == (i & 1)) {
				if   (j > 0) f[j][sf[j]++] = x, belong[x] = j;
				else j = -j, g[j][sg[j]++] = x, belong[x] = -j;
			} else {
				if   (j > 0) g[j][sg[j]++] = x, belong[x] = -j;
				else j = -j, f[j][sf[j]++] = x, belong[x] = j;
			}
		}

		memset(dp, 0, sizeof(dp)); dp[0][0] = 1;
		memset(which, 0, sizeof(which));
		for (k = 1; k < sz; k++) {
			for (i = 0; i <= p; i++) {
				if (dp[k-1][i] == 0) continue;
				j = i + sf[k], dp[k][j] += dp[k-1][i];
				if (dp[k][j] > 2) dp[k][j] = 2;
				which[k][j] = k;
				j = i + sg[k], dp[k][j] += dp[k-1][i];
				if (dp[k][j] > 2) dp[k][j] = 2;
				which[k][j] = -k;
			}
		}
		if (dp[sz-1][p1] == 2) { puts("no"); continue; }

		memset(divine, 0, p); k = p1;
		for (i = sz-1; i >= 1; i--) {
			if (which[i][k] > 0) {
				for (j = 0; j < sf[i]; j++) divine[f[i][j]] = 1;
				k -= sf[i];
			} else {
				for (j = 0; j < sg[i]; j++) divine[g[i][j]] = 1;
				k -= sg[i];
			}
		}

		for (i = 0; i < p; i++) if (divine[i]) printf("%d\n", i+1);
		puts("end");
	}
	return 0;
}

