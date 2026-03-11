// AOJ 3065 How old are you
// 2019.9.30 bal4u

#include <stdio.h>

typedef long long ll;

int main()
{
	int N, q, x;
	ll A, B;

	A = 0, B = 1;
	scanf("%d", &N);
	while (N--) {
		scanf("%d%d", &q, &x);
		if (q == 1) A *= x, B *= x;
		else if (q == 2) A += x;
		else A -= x;
	}
	printf("%lld %lld\n", -A, B);
	return 0;
}
