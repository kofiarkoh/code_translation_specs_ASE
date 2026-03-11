// Aizu 3018: nCm
// 2017.10.25 bal4u@uu
#include <stdio.h>
int main()
{
	long long n;
	scanf("%lld", &n); n++;
	printf("%lld\n", n & -n);
	return 0;
}