#include <stdio.h>

int main()
{
	int i, N, A[2001], sum = 0;
	scanf("%d", &N);
	for (i = 1; i <= N; i++) {
		scanf("%d", &(A[i]));
		sum += A[i];
	}
	
	int j, k, r, half = sum / 2, halff = half / 64;
	const unsigned long long sup = 18446744073709551615;
	unsigned long long flag[32000] = {1}, tmp[32000];
	for (i = 1; i <= N; i++) {
		k = A[i] / 64;
		r = A[i] % 64;
		for (j = 0; j <= halff; j++) tmp[j] = 0;
		for (j = 0; j <= halff - k; j++) {
			tmp[j+k] |= (flag[j] & (sup >> r)) << r;
			tmp[j+k+1] |= flag[j] >> (64 - r);
		}
		for (j = 0; j <= halff; j++) flag[j] |= tmp[j];
	}
	
	flag[halff] &= (sup >> (63 - half % 64));
	for (i = halff; i >= 0; i--) if (flag[i] != 0) break;
	for (j = -1; flag[i] != 0; j++, flag[i] >>= 1);
	printf("%d\n", sum - i * 64 - j);
	fflush(stdout);
	return 0;
}