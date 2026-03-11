// Pana2020 D
// 2020.3.14 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define pc(c) putchar_unlocked(c)

int main()
{
	int i, j, k, N;

	scanf("%d", &N);
	k = (1 << N) / 2;
	for (i = 0; i < k; ++i) {
		for (j = N-1; j >= 0; --j) {
			pc('a'+ ((i & (1<<j)) != 0));
		}
		pc('\n');
	}
	return 0;
}
