// AOJ DPL_5_H Balls and Boxes 8
// n 個の区別できるボールを k 個の区別できない箱に入れる
// とき、可能な入れ方の総数を求めてください。
// ただし、
//    どのボールも、必ずいずれかの箱に入れる。
//    各箱に入れることのできるボールの数は高々1つまで。
// 2019.3.11 bal4u

#include <stdio.h>
#include <stdlib.h>

int main()
{
	int n, k;

	scanf("%d%d", &n, &k);
	puts(n <= k ? "1" : "0");
	return 0;
}
