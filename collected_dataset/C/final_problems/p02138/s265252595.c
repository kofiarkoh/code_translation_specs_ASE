// AOJ 3040 U and U
// 2019.4.2 bal4u

#include <stdio.h>

int main()
{
	long long N, M;

	scanf("%lld%lld", &N, &M);
	if (N >= (M << 1)) puts("0");
	else if (M >= (N << 1)) puts("1");
	else if (M <= N) puts("2");
	else if ((M<<1)+M+(M>>1) >= (N<<2)) puts("3");
	else puts("4");
	return 0;
}
