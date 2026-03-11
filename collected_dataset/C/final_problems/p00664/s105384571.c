// AOJ 1079 Cosmic Market
// 2018.2.3 bal4u

#include <stdio.h>
#include <string.h>

int a[50001], b[50001], ord[50001];
char mk[2][50001];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int in1()
{
	int c = getchar_unlocked() & 1;
	getchar_unlocked();
	return c;
}
int main()
{
	int R, C, Q, i, r, c;
	long long ans;

	while (R = in()) {
		C = in(), Q = in();
		for (i = 0; i < Q; i++)	a[i] = in1(), b[i] = in(), ord[i] = in1();

		memset(mk, 0, sizeof(mk));
		ans = 0; r = c = 0;
		i = Q; while (i--) {
			if (a[i]) {
				if (mk[0][b[i]]) continue;
				if (ord[i]) ans += R - r;
				mk[0][b[i]] = 1;
				c++;
			} else {
				if (mk[1][b[i]]) continue;
				if (ord[i]) ans += C - c;
				mk[1][b[i]] = 1;
				r++;
			}
		}
		printf("%lld\n", ans);
	}
	return 0;
}
