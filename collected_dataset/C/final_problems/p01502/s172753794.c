// AOJ 2386: Sightseeing Tour
// 2018.1.2 bal4u@uu

#include <stdio.h>

int map[102][102];

//#define getchar_unlocked()  getchar()
int in()
{
	int n, c;

	while ((c = getchar_unlocked()) < '0');
	n = 0;
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, r, c, x;
	long long ans;

	n = in();
	for (r = 0; r < n; r++) for (c = 0; c < n; c++) map[r][c] = in();

	ans = 0;
	for (r = 0; r < n; r++) for (c = r+1; c < n; c++) {
		x = map[r][c];
		if (x > map[c][r]) x = map[c][r];
		ans += x;
	}
	printf("%lld\n", ans);
	return 0;
}

