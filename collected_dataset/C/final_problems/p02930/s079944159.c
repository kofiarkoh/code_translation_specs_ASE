#include <stdio.h>

int main()
{
	int N;
	scanf("%d", &N);
	
	const int bit[10] = {1, 2, 4, 8, 16, 32, 64, 128, 256, 512};
	int i, j, k;
	for (i = 0; i < N - 1; i++) {
		for (j = i + 1; j < N - 1; j++) {
			for (k = 0; k < 10; k++) if ((i & bit[k]) != (j & bit[k])) break;
			printf("%d ", k + 1);
		}
		for (k = 0; k < 10; k++) if ((i & bit[k]) != ((N - 1) & bit[k])) break;
		printf("%d\n", k + 1);
	}
	
	fflush(stdout);
	return 0;
}