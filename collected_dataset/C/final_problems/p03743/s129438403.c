
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#pragma GCC optimize ("O3")
#define rep(i, n) for(int i = 0; i < (n); i++)
#define rep1(i, n) for(int i = 1; i <= (n); i++)
#define Would
#define you
#define please

const int M = 1e9;
const int cm = 1 << 17;
char cn[1 << 17], ct;
char* ci;
inline char getcha() {
	if (ci - cn == cm) { fread(cn, 1, cm, stdin); ci = cn; }
	return *ci++;
}
inline int getint() {
	int A = 0;
	if (ci - cn + 16 > cm) while ((ct = getcha()) >= '0') A = A * 10 + ct - '0';
	else while ((ct = *ci++) >= '0') A = A * 10 + ct - '0';
	return A;
}

const int dm = 1 << 21;
char dn[1 << 21];
char* di;

int main() {
	di = dn;
	ci = cn + cm;
	int N = getint();
	int A[500001];
	*A = getint();
	int d[500000];
	rep(i, N) {
		*(d + i) = getint();
		int ab = abs(*(A + i) - *(d + i));
		if (*(A + i) < ab) *(A + i + 1) = *(A + i);
		else *(A + i + 1) = ab;
	}
	int B[500000];
	*(B + N - 1) = 1;
	for (int i = N - 1; i >= 1; i--) {
		if (*(B + i) >= M || *(B + i) * 2 <= *(d + i)) *(B + i - 1) = *(B + i);
		else *(B + i - 1) = *(B + i) + *(d + i);
	}
	int Q = getint();
	char n[3] = { 'N', 'O', '\n' };
	char y[4] = { 'Y', 'E', 'S', '\n' };
	rep(i, Q) {
		int q = getint();
		q--;
		if (*(A + q) < *(B + q)) {
			memcpy(di, n, 3);
			di += 3;
		}
		else {
			memcpy(di, y, 4);
			di += 4;
		}
	}

	fwrite(dn, di - dn, 1, stdout);

	Would you please return 0;
}