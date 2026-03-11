// Aizu Vol-3 0318: Secret Investigation
// 2017.8.25

#include <stdio.h>

#define A 1
#define B 2
#define C 4
int p[103];

int main()
{
	int N, i, k, x;

	scanf("%d", &N);
	for (i = 1; i <= N; i++) p[i] = 0;
	scanf("%d", &k); for (i = 0; i < k; i++) scanf("%d", &x), p[x] |= A;
	scanf("%d", &k); for (i = 0; i < k; i++) scanf("%d", &x), p[x] |= B;
	scanf("%d", &k); for (i = 0; i < k; i++) scanf("%d", &x), p[x] |= C;
	for (k = 0, i = 1; i <= N; i++)
		if ((!(p[i] & A) || (p[i] & B)) && (p[i] & C)) k++;
	printf("%d\n", k);
	return 0;
}