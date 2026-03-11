#include <stdio.h>
#include <math.h>

long long power(long long n, int k)
{
	if (k == 0 || n == 1) return 1;
	
	long long i, N;
	for (i = 1, N = n; i * 2 <= k; i *= 2) N = N * N;
	return N * power(n, k - i);
}

int main()
{	
	long long N, A;
	scanf("%lld %lld", &N, &A);
	
	int k, l;
	long long n, min = N;
	for (k = 2, n = (long long)sqrt(N); n > 1 && A * (k - 1) < min; k++) {
		for (; pow(n, k) > N; n--);
		for (l = 1; l <= k && pow(n, k - l) * pow(n + 1, l) < N; l++);
		if (A * (k - 1) + n * k + l < min) min = A * (k - 1) + n * k + l;
	}
	
	printf("%lld\n", min);
	fflush(stdout);
	return 0;
}