// AOJ 3027: OOllOll
// 2018.3.28 bal4u

#include <stdio.h>

int main()
{
	int N, i;

	scanf("%d", &N);
	if (N == 1) { puts("1"); return 0; }
	for (i = 0; (1<<i) < N; i++);
	if ((1<<i)-1 > N) i--; 
	printf("%d\n", i);
	return 0;
}
