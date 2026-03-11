#include <stdio.h>
#include <inttypes.h>

/*
うーん、わからん。
まーいっぱいぶん回せば、ビット的に考えてどっかで止まるんじゃね？知らんけど。
*/

int main(void) {
	uint64_t A, B, C;
	int mareitaso = 0;
	if (scanf("%" SCNu64 "%" SCNu64 "%" SCNu64, &A, &B, &C) != 3) return 1;
	while (A % 2 == 0 && B % 2 == 0 && C % 2 == 0 && mareitaso < 1000000) {
		uint64_t na, nb, nc;
		na = B / 2 + C / 2;
		nb = A / 2 + C / 2;
		nc = A / 2 + B / 2;
		A = na;
		B = nb;
		C = nc;
		mareitaso++;
#if 0
		printf("%" PRIu64 " %" PRIu64 " %" PRIu64 "\n", A, B, C);
#endif
	}
	printf("%d\n", A % 2 != 0 || B % 2 != 0 || C % 2 != 0 ? mareitaso : -1);
	return 0;
}

/*

A                          B                          C
      B/2   +C/2    A/2         +C/2    A/2   +B/2
A/2  +B/4   +C/4    A/4   +B/2  +C/4    A/4   +B/4   +C/2
A/4  +B*3/8 +C*3/8  A*3/8 +B/4  +C*3/8  A*3/8 +B*3/8 +C/4
A*3/8+B*5/16+C*5/16 A*5/16+B*3/8+C*5/16 A*5/16+B*5/16+C*3/8

*/
