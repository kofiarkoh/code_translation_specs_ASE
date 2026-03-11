// AOJ 2538 Stack Maze
// 2018.2.2 bal4u

#include <stdio.h>
#include <string.h>
#include <ctype.h>

char map[53][53];
char mk[53][53][53][53];
int memo[53][53][53][53];
char a[53][53][11][2], hi[53][53];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int calc(int lr, int lc, int rr, int rc)
{
	int i, r, c, t, ans;

	if ((ans = memo[lr][lc][rr][rc]) >= 0) return ans;
	if (lc > rc || lr > rr) { ans = 0; goto done; }
	if (map[lr][lc] == '#' || map[rr][rc] == '#' ||
		!mk[lr][lc][rr][rc]) { ans = -1; goto done; }

	ans = 0;
	for (i = 0; i < hi[lr][lc]; i++) {
		r = a[lr][lc][i][0], c = a[lr][lc][i][1];
		if (r < lr || r > rr || c < lc || c > rc) continue;
		t = calc(lr+1, lc, r-1, c) + 1 + calc(r, c, rr, rc); if (t > ans) ans = t;
		t = calc(lr, lc+1, r-1, c) + 1 + calc(r, c, rr, rc); if (t > ans) ans = t;
		t = calc(lr+1, lc, r, c-1) + 1 + calc(r, c, rr, rc); if (t > ans) ans = t;
		t = calc(lr, lc+1, r, c-1) + 1 + calc(r, c, rr, rc); if (t > ans) ans = t;
	}
	t = calc(lr+1, lc, rr, rc); if (t > ans) ans = t;
	t = calc(lr, lc+1, rr, rc); if (t > ans) ans = t;
done:
	return memo[lr][lc][rr][rc] = ans;;
}

int main()
{
	int H, W, r, c, u, v, k;

	while (H = in()) {
		W = in();
		memset(mk, 0, sizeof(mk));
		memset(hi, 0, sizeof(hi));
		memset(memo, -1, sizeof(memo));

		for (r = 1; r <= H; r++) {
			fgets(map[r]+1, 53, stdin);
			for (c = 1; c <= W; c++) if (map[r][c] != '#') {
				mk[r][c][r][c] = 1;
				for (u =1 ; u <= r; u++) for (v = 1; v <= c; v++) {
					if (mk[u][v][r-1][c] || mk[u][v][r][c-1]) {
						mk[u][v][r][c] = 1;
						if (isupper(map[r][c]) && islower(map[u][v]) &&
							(map[r][c] & 0x1f) == (map[u][v] & 0x1f)) {
							k = hi[u][v]++;
							a[u][v][k][0] = r, a[u][v][k][1] = c;
						}
					}
				}
			}
		}

		printf("%d\n", calc(1, 1, H, W));
	}
	return 0;
}
