// AOJ 1603: 500-yen Saving
// 2018.1.30 bal4u

#include <stdio.h>
#include <string.h>

#define MAX  49900

int p[101];
char dc[101][50000]; int ds[101][50000];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, i, k, ii, j, c, s, a, b, f;

	while (n = in()) {
		for (i = 0; i < n; i++) p[i] = in();
		memset(dc, -1, sizeof(dc)), memset(ds, 0, sizeof(ds));
        dc[0][0] = 0;

		for (i = 0, ii = 1; i < n; i++, ii++) for (j = 0; j <= MAX; j++) if (dc[i][j] >= 0) {
            if (dc[ii][j] < dc[i][j] || dc[i][j] == dc[ii][j] && ds[i][j] < ds[ii][j]) {
                dc[ii][j] = dc[i][j], ds[ii][j] = ds[i][j];
            }
            a = dc[i][j], b = ds[i][j] + p[i];
            f = (5000 - p[i]) % 1000;
			k = j + f; if (k >= 500) a++, k -= 500;
            if (dc[ii][k] < a || dc[ii][k] == a && b < ds[ii][k]) dc[ii][k] = a, ds[ii][k] = b;
        }
        c = s = 0;
		for (j = 0; j <= MAX; j++) {
			if (c < dc[n][j] || dc[n][j] == c && ds[n][j] < s)
				c = dc[n][j], s = ds[n][j];
		}
        printf("%d %d\n", c, s);
	}
	return 0;
}
