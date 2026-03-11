#include <stdio.h>

int main(void)
{
	int N, D, i;
	scanf("%d", &N);
	scanf("%d", &D);
	D = 2 * D + 1;//監視範囲
	i = N / D;
	if (N%D != 0) {
		i++;
	}
	printf("%d", i);
}