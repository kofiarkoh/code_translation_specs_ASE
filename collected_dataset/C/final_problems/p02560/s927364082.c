#include <stdio.h>

long long floor_sum(long long n, long long m, long long a, long long b)
{
	if (a >= m || b >= m) return n * (n - 1) * (a / m) / 2 + n * (b / m) + floor_sum(n, m, a % m, b % m);
	long long y = (n * a + b) / m, x = m * y - b;
	if (y == 0) return 0;
	else return (n - (x + a - 1) / a) * y + floor_sum(y, a, m, (a - x % a) % a);
}

int main()
{
	int t, T, N, M, A, B;
	scanf("%d", &T);
	for (t = 1; t <= T; t++) {
		scanf("%d %d %d %d", &N, &M, &A, &B);
		printf("%lld\n", floor_sum(N, M, A, B));
	}
	fflush(stdout);
	return 0;
}