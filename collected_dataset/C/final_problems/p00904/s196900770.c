#include <stdio.h>

/* <m, n> が <p, q> の約数であるか判定する */
int isDivisor(int p, int q, int m, int n) {
	int x = m * m + n * n;
	/* ゼロ除算 */
	if (x == 0) return 0;
	/* <1, 0>, <0, 1>, <-1, 0>, <0, -1>, <m, n>, <-n, m>, <-m, -n>, <n, -m> */
	if (x == 1 || x == p * p + q * q) return 0;
	/* 判定 */
	return (m * p + n * q) % x == 0 && (m * q - n * p) % x == 0;
}

int main(void) {
	int l, p, q; /* 入力 */

	/* データセット毎の処理 */
	for (scanf("%d", &l); l > 0 ; l--) {
		scanf("%d%d", &p, &q);

		/*
			与えられたイチョウ数 <p, q> が任意のイチョウ数 <m, n> で割り切れるかを調べる。
			<m, n> * <x, y> = <p, q> なら (m^2 + n^2)(x^2 + y^2) = p^2 + q^2 である。
			x^2 + y^2 > 0 なので <m, n> は m^2 + n^2 <= p^2 + q^2 を満たす。
		*/
		int prime = 1;
		int m, n;
		for (m = 0; m <= p * p + q * q; m++) for (n = 0; n <= p * p + q * q - m * m; n++) {
			if (isDivisor(p, q, m, n)) {
				prime = 0;
				goto _break;
			}
		}
		_break:;

		if (prime) printf("%c\n", 'P');
		else printf("%c\n", 'C');
	}

	return 0;
}