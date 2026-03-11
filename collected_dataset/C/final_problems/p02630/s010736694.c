#include <stdio.h>

int main()
{
	int N, Q;
	scanf("%d", &N);
	int A[100001] = {0};
	long long sum = 0;
	while (N--)
	{
		int a;
		scanf("%d", &a);
		A[a]++;
		sum += a;
	}
	scanf("%d", &Q);
	while (Q--)
	{
		int b, c;
		scanf("%d%d", &b, &c);
		sum += A[b] * (c - b);
		A[c] += A[b];
		A[b] = 0;
		printf("%d\n", sum);
	}
}