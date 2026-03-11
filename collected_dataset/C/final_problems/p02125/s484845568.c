// AOJ 3027: OOllOll
// 2018.3.28 bal4u

#include <stdio.h>

int main()
{
	int N, i;
	scanf("%d", &N);
	for (i = 1; (1<<i)-1 < N; i++);
	if (N < (1<<i)-1) i--; 
	printf("%d\n", i);
	return 0;
}
