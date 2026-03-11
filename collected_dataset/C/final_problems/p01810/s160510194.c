// AOJ 2707: Jail
// 2017.10.9 bal4u@uu

#include <stdio.h>

#define MAX 1000000000000000000LL
int n, k;

int check(long long m)
{
	int i;
	for (i = 0; i < n-1; i++) {
		m -= (m - 1) / k + 1;
        if (m <= 0) return 1;
    }
    return 0;
}

long long bsch()
{
	long long m, l = 0, r = MAX;
    while (l+1 < r) {
        m = (l + r) >> 1;
        if (check(m)) l = m; else r = m;
    }
	return l;
}

int main()
{
	scanf("%d%d", &n, &k);
	printf("%lld\n", bsch());
	return 0;
}