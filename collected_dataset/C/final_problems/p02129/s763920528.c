// AOJ 3031 Ghost Legs
// 2018.3.29 bal4u

#include <stdio.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

char p[3][3][3];

int main()
{
	int N, w, i, ans;
	int a[3], t;

	N = in();
	for (i = 0; i < N; i++) {
		a[0] = 0, a[1] = 1, a[2] = 2;
		w = in();
		while (w--) {
			if (in()) t = a[1], a[1] = a[2], a[2] = t;
			else      t = a[0], a[0] = a[1], a[1] = t;
		}
//printf("--> %d %d %d\n", a[0], a[1], a[2]);
		p[a[0]][a[1]][a[2]]++;
	}

	ans = p[0][1][2] ||
		  p[0][2][1] >= 2 || p[1][0][2] >= 2 || p[2][1][0] >= 2 ||
		  p[1][2][0] >= 3 || p[2][0][1] >= 3 || (p[1][2][0] && p[2][0][1]) ||
		  ((p[0][2][1] && p[1][0][2] || p[0][2][1] && p[2][1][0] ||
			     p[1][0][2] && p[2][1][0]) && (p[1][2][0] || p[2][0][1]));

	puts(ans? "yes": "no");
	return 0;
}
