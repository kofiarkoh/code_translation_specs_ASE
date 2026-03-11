// AOJ 2686: Unfair Game
// 2018.2.27

#include <stdio.h>

int s[100002], N;
int a[5];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

long long nim(long long x)
{
	long long t = 0;
	int i = N; while (i--) t ^= s[i] % x;
	return t;
}

int main()
{
	int A, B, i, j, f;
	long long t, x, y;

	N = in(), A = in(), B = in();
	for (i = 0; i < N; i++) s[i] = in();

	if (A >= B) {
		f = (nim(B+1) != 0);
		if (A > B && !f) for (i = 0; i < N; i++) if (s[i] > B) { f = 1; break; }
	} else {
		f = ((t = nim(A+1)) != 0);
		j = 0; for (i = 0; i < N && j < 2; i++) { if (s[i] > A) a[j++] = i; }
		if (f) {
			if (j > 1) f = 0;
			else if (j == 1) {
				x = (s[a[0]] % (A+1)) ^ t;
				y = s[a[0]] - x;
				if (y <= 0 || A < y || x > A) f = 0;
			}
		}
    }
    puts(f? "Hanako": "Jiro");
	return 0;
}
