#include <stdio.h>

int main()
{
	int i, N;
	long long A[301], xor = 0;
	scanf("%d", &N);
	for (i = 1; i <= N; i++) {
		scanf("%lld", &(A[i]));
		xor ^= A[i];
	}
	if (xor == 0) {
		printf("0\n");
		fflush(stdout);
		return 0;
	} else xor ^= A[1] ^ A[2];
	
	long long bit[41], x = A[1] + A[2], y = x - xor;
	for (i = 1, bit[0] = 1; i <= 40; i++) bit[i] = bit[i-1] << 1;
	if (y < 0 || x < y || y % 2 == 1 || y / 2 > A[1]) {
		printf("-1\n");
		fflush(stdout);
		return 0;
	} else y /= 2;
	
	int flag[41];
	long long a = y, b = y, c = y << 1;
	for (i = 0; i <= 40; i++) flag[i] = ((y & bit[i]) > 0)? 1: 0; 
	for (i = 0; i <= 40; i++) {
		if ((x & bit[i]) == (c & bit[i])) continue;
		else if (flag[i] == 1) break;
		flag[i] = -1;
		c += bit[i];
	}
	if (c != x) {
		printf("-1\n");
		fflush(stdout);
		return 0;
	}
	
	for (i = 40; i >= 0; i--) if (flag[i] == -1 && a + bit[i] <= A[1]) a += bit[i];
	if (a > 0) printf("%lld\n", A[1] - a);
	else printf("-1\n");
	fflush(stdout);
	return 0;
}