#include <stdio.h>

void add_BIT(int N, long long BIT[], int i, int k)
{
	while (i <= N) {
		BIT[i] += k;
		i += (i & -i);
	}
}

long long sum_BIT(long long BIT[], int r)
{
	long long sum = 0;
	while (r > 0) {
		sum += BIT[r];
		r -= (r & -r);
	}
	return sum;
}

int main()
{
	int i, N, Q, a, t, p, x;
	long long BIT[500001] = {};
	scanf("%d %d", &N, &Q);
	for (i = 1; i <= N; i++) {
		scanf("%d", &a);
		add_BIT(N, BIT, i, a);
	}	
	for (i = 1; i <= Q; i++) {
		scanf("%d %d %d", &t, &p, &x);
		if (t == 0) add_BIT(N, BIT, p + 1, x);
		else printf("%lld\n", sum_BIT(BIT, x) - sum_BIT(BIT, p));
	}
	fflush(stdout);
	return 0;
}