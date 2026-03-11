// AOL 1059 Mysterious Onslaught
// 2018.2.18 bal4u

#include <stdio.h>
#include <string.h>

int n;
int map[5][5][5][5][5];
short memo[5][1<<25];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int calc(int b)
{
	int r1, c1, r2, c2, k, ans;

	if (!b) return 0;
	if (memo[n-1][b] >= 0) return memo[n-1][b];

	for (r1 = 0; r1 < n; r1++) for (c1 = 0; c1 < n; c1++) {
		if (b & (1<<(r1*n + c1))) goto next;
	}

next:
	ans = n*n;
	for (r2 = r1; r2 < n; r2++) for (c2 = c1; c2 < n; c2++) {
		k = b ^ map[n-1][r1][c1][r2][c2];
		k = calc(k) + 1;
		if (ans > k) ans = k;
	}

	return memo[n-1][b] = ans;
}

void init()
{
	int n, r1, r2, c1, c2, r, c;

	for (n = 1; n <= 5; n++)
		for (r1 = 0; r1 < n; r1++) for (c1 = 0; c1 < n; c1++)
			for (r2 = r1; r2 < n; r2++) for (c2 = c1; c2 < n; c2++)
				for (r = r1; r <= r2; r++) for (c = c1; c <= c2; c++)
					map[n-1][r1][c1][r2][c2] |= (1<<(r*n+c));
}

int main()
{
	int r, c, b, k;

	init();
	memset(memo, -1, sizeof(memo));
	while (n = in()) {
		b = 0;
		for (k = r = 0; r < n; r++, k += n) for (c = 0; c < n; c++) {
			if (getchar_unlocked() & 1) b |= 1 << (k+c);
			getchar_unlocked();
		}
		k = calc(b);
		while (k--) printf("myon");
		putchar('\n');
	}
	return 0;
}
